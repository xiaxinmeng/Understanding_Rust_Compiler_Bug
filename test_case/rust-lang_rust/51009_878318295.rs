console
$ cat main.rs 
pub extern "C" fn f() -> i32 { 92 }
$ rustc -C lto=fat --crate-type rlib main.rs
$ rustc -C lto=fat --crate-type cdylib main.rs
$ rustc -C lto=fat --crate-type rlib,cdylib main.rs
error: lto can only be run for executables, cdylibs and static library outputs

error: aborting due to previous error
$ rustc --version
rustc 1.54.0-beta.1 (bf62f4de3 2021-06-23)
