use std::{io::Error, path::PathBuf, ptr};
use widestring::WideCString;
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

fn path_to_c_str(path: PathBuf) -> WideCString {
  let absolute_path = path.canonicalize().unwrap();
  let absolute_path = absolute_path.to_str().unwrap();
  let absolute_path = if absolute_path.starts_with("\\\\?\\") {
    &absolute_path[4..]
  } else {
    absolute_path
  };

  let absolute_path: Vec<u16> = absolute_path.encode_utf16().collect();

  WideCString::new(absolute_path).unwrap()
}

fn get_item_id_list(path: PathBuf) -> Result<PIDLIST_ABSOLUTE, Error> {
  let c_str = path_to_c_str(path);

  let item_id_list = unsafe { ILCreateFromPathW(c_str.as_ptr()) };
  if item_id_list.is_null() {
    Err(Error::last_os_error())
  } else {
    Ok(item_id_list)
  }
}

pub fn that(path: PathBuf) -> Result<(), Error> {
  unsafe { CoInitialize(ptr::null_mut()) };

  let item_id_list = get_item_id_list(path)?;
  let ret = unsafe { SHOpenFolderAndSelectItems(item_id_list, 0, ptr::null_mut(), 0) };
  unsafe { ILFree(item_id_list) };

  if ret != S_OK {
    Err(Error::from_raw_os_error(ret))
  } else {
    Ok(())
  }
}

pub fn those(path: PathBuf, mut items: Vec<PathBuf>) -> Result<(), Error> {
  unsafe { CoInitialize(ptr::null_mut()) };

  let item_id_list = get_item_id_list(path)?;
  let item_id_list_items: Result<Vec<_>, _> = items.drain(..).map(get_item_id_list).collect();
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
    Err(Error::from_raw_os_error(ret))
  } else {
    Ok(())
  }
}

#[test]
fn test_that() {
  that(".".parse().unwrap()).unwrap();
}

#[test]
fn test_those() {
  those(
    ".".parse().unwrap(),
    vec!["Cargo.toml".parse().unwrap(), "src".parse().unwrap()],
  )
  .unwrap();
}
