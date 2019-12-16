mod error;
mod helpers;
mod windows;

pub use crate::windows::{that, those};

#[test]
fn test_that() {
  that(".").unwrap();
}

#[test]
fn test_those() {
  those(".", vec!["Cargo.toml", "src"]).unwrap();
}
