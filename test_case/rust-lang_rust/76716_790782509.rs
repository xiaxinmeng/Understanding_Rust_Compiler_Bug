
$ cat hidden.rs 
//! some docs
#![warn(missing_docs)]
#[doc(hidden)]
pub fn f() {}
