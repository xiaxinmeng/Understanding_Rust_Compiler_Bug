
$ rust foo.rs --crate-type lib
$ rustc bar.rs -C lto --extern foo=libfoo.rlib
