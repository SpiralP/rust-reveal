use crate::error::*;
use std::{path::Path, process::Command};

pub fn that<P: AsRef<Path>>(path: P) -> Result<()> {
  let path = path.as_ref();

  let absolute_path = path.canonicalize()?;
  let absolute_path_parent = absolute_path.parent().unwrap_or(&absolute_path);

  let absolute_path = absolute_path
    .to_str()
    .chain_err(|| "absolute_path.to_str")?;
  let absolute_path_parent = absolute_path_parent
    .to_str()
    .chain_err(|| "absolute_path_parent.to_str")?;

  match Command::new("nautilus")
    .args(&["--select", absolute_path])
    .status()
  {
    Ok(status) => {
      if status.success() {
        Ok(())
      } else {
        Err(format!("nautilus failed with {:?}", status).into())
      }
    }

    Err(e) => match Command::new("xdg-open").arg(absolute_path_parent).status() {
      Ok(status) => {
        if status.success() {
          Ok(())
        } else {
          Err(format!("xdg-open failed with {:?}", status).into())
        }
      }

      Err(e2) => Err(
        format!(
          "Both nautilus ({}) and xdg-open ({}) couldn't be run",
          e, e2
        )
        .into(),
      ),
    },
  }
}

pub fn those<P: AsRef<Path>>(path: P, items: Vec<P>) -> Result<()> {
  that(items.get(0).unwrap_or(&path))
}
