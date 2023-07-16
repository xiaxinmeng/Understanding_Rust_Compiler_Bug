
$ cat a.rs
#![crate_type = "dylib"]
#[no_mangle]
pub extern "stdcall" fn std_func() {}

$ cat a.def
LIBRARY a
EXPORTS
  std_func=std_func@0

$ rustc a.rs -C link-args="a.def"
