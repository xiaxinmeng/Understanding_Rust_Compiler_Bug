
$ cat bar.rs
pub struct A(String);
$ cat foo.rs
pub fn a(_: bar::A) {}
$ rustc bar.rs --crate-type rlib
$ rustc foo.rs --crate-type rlib --extern bar=libbar.rlib --emit llvm-ir
$ wc -l foo.ll
980 foo.ll
