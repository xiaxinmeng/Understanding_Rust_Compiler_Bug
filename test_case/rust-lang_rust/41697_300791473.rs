
$ rustc --version
rustc 1.19.0-nightly (978d2cfee 2017-05-10)
$ rustc -Z dump-mir=all src/test/run-pass/issue-33387.rs
error[E0391]: unsupported cyclic reference between types/traits detected
[...]
