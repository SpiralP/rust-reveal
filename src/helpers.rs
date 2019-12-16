use crate::error::*;
use std::path::Path;
use widestring::WideCString;

pub fn path_to_c_str<P: AsRef<Path>>(path: P) -> Result<WideCString> {
  let path = path.as_ref();

  let absolute_path = path.canonicalize()?;
  let absolute_path = absolute_path
    .to_str()
    .chain_err(|| "absolute_path.to_str")?;
  let absolute_path = if absolute_path.starts_with("\\\\?\\") {
    &absolute_path[4..]
  } else {
    absolute_path
  };

  let absolute_path: Vec<u16> = absolute_path.encode_utf16().collect();

  Ok(WideCString::new(absolute_path).chain_err(|| "WideCString::new(absolute_path)")?)
}
