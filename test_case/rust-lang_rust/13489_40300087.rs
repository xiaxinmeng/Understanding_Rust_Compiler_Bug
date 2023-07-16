
$ cat foo.rs
#![crate_type = "dylib"]

fn main() {}
$ rustc --crate-file-name foo.rs --crate-type=rlib 
libfoo-aab0fa9e-0.0.dylib
libfoo-aab0fa9e-0.0.rlib
