rust
use std::ffi::OsStr;
static NONE: (dyn AsRef<OsStr>, u8) = ("hello", 42);
