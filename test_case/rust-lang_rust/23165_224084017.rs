
$ cat foo.rs
fn main() {}

$ rustc --emit=dep-info foo.rs 
$ cat foo.d 
foo.d: foo.rs

foo.rs:
$ rustc --version
rustc 1.9.0 (e4e8b6668 2016-05-18)
