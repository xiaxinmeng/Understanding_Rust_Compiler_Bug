 bash
$ rustc plugin_lib.rs --crate-type=dylib
$ rustc plugin_test.rs -L .
Segmentation fault
$ rustc --version
rustc 1.7.0-nightly (2370d461a 2015-12-30)
