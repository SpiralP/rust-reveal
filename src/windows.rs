use crate::{error::*, helpers::path_to_c_str};
use std::{io, path::Path, ptr};
use winapi::{
  shared::{
    minwindef::{DWORD, UINT},
    ntdef::{HRESULT, PCWSTR},
    winerror::S_OK,
  },
  um::{
    objbase::CoInitialize,
    shtypes::{PCIDLIST_ABSOLUTE, PCUITEMID_CHILD_ARRAY, PIDLIST_ABSOLUTE, PIDLIST_RELATIVE},
  },
};

#[link(name = "Shell32")]
extern "C" {
  fn ILCreateFromPathW(pszPath: PCWSTR) -> PIDLIST_ABSOLUTE;

  fn ILFree(pidl: PIDLIST_RELATIVE);

  fn SHOpenFolderAndSelectItems(
    pidlFolder: PCIDLIST_ABSOLUTE,
    cidl: UINT,
    apidl: PCUITEMID_CHILD_ARRAY,
    dwFlags: DWORD,
  ) -> HRESULT;
}

fn get_item_id_list<P: AsRef<Path>>(path: P) -> Result<PIDLIST_ABSOLUTE> {
  let c_str = path_to_c_str(path)?;

  let item_id_list = unsafe { ILCreateFromPathW(c_str.as_ptr()) };
  if item_id_list.is_null() {
    Err(io::Error::last_os_error().into())
  } else {
    Ok(item_id_list)
  }
}

pub fn that<P: AsRef<Path>>(path: P) -> Result<()> {
  unsafe { CoInitialize(ptr::null_mut()) };

  let item_id_list = get_item_id_list(path)?;
  let ret = unsafe { SHOpenFolderAndSelectItems(item_id_list, 0, ptr::null_mut(), 0) };
  unsafe { ILFree(item_id_list) };

  if ret != S_OK {
    Err(io::Error::from_raw_os_error(ret).into())
  } else {
    Ok(())
  }
}

pub fn those<P: AsRef<Path>>(path: P, mut items: Vec<P>) -> Result<()> {
  unsafe { CoInitialize(ptr::null_mut()) };

  let item_id_list = get_item_id_list(path)?;
  let item_id_list_items: Result<Vec<_>> = items.drain(..).map(get_item_id_list).collect();
  let item_id_list_items = item_id_list_items?;

  let ret = unsafe {
    SHOpenFolderAndSelectItems(
      item_id_list,
      item_id_list_items.len() as _,
      item_id_list_items.as_ptr() as _,
      0,
    )
  };

  for ptr in item_id_list_items {
    unsafe { ILFree(ptr) };
  }

  unsafe { ILFree(item_id_list) };

  if ret != S_OK {
    Err(io::Error::from_raw_os_error(ret).into())
  } else {
    Ok(())
  }
}
