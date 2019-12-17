mod error;

cfg_if::cfg_if! {
  if #[cfg(windows)] {
    mod windows;
    pub use crate::windows::{that, those};
  } else if #[cfg(unix)] {
    mod unix;
    pub use crate::unix::{that, those};
  }
}

#[test]
fn test_that() {
  that(".").unwrap();
}

#[test]
fn test_those() {
  those(".", vec!["Cargo.toml", "src"]).unwrap();
}
