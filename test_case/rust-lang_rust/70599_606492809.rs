shell
$ rustup toolchain install nightly-2020-03-18 --profile minimal
$ cargo new foo
$ cd foo
$ # paste above code into src/main.rs
$ cargo +nightly-2020-03-18 b
