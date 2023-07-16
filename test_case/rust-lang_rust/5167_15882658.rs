
$ cat empty-struct.rs
struct A {}
fn main() {}

$ rustc empty-struct.rs
empty-struct.rs:1:10: 1:11 error: Unit-like struct should be written as: struct A;
empty-struct.rs:1 struct A {}
                            ^
