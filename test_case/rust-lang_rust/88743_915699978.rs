rust
#[cfg(unix)]
mod unix {
  #[doc(cfg(unix))]
  pub struct Foo {}
}

#[cfg(windows)]
mod windows {
  #[doc(cfg(windows)]
  pub struct Foo {}
}

#[cfg(unix)]
pub use unix::Foo;
#[cfg(windows)]
pub use windows::Foo;
