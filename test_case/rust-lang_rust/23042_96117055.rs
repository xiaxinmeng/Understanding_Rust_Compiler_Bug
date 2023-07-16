
~/Downloads $ rustc test.rs
test.rs:8:13: 8:16 error: the value of the associated type `Bar` (from the trait `Foo`) must be specified [E0191]
test.rs:8 fn next(i: &Foo) {
                      ^~~
error: aborting due to previous error
