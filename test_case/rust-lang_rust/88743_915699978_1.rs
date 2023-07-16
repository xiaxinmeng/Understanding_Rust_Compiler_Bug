rust
#[cfg(unix)]
pub use inner::UnixFoo;
#[cfg(windows)]
pub use inner::WindowsFoo;
