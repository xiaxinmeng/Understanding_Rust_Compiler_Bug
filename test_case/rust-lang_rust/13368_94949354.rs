
$ rustc --version
rustc 1.0.0-dev (f46c4e158 2015-04-20) (built 2015-04-20)
$ rustc src/bin/main.rs
Segmentation fault: 11
$ rustc src/bin/main.rs
Segmentation fault: 11
$ rustc src/bin/main.rs
Segmentation fault: 11
$ RUST_BACKTRACE=1 rustc src/bin/main.rs
Illegal instruction: 4
$ RUST_BACKTRACE=1 rustc src/bin/main.rs
Segmentation fault: 11
$ RUST_BACKTRACE=1 rustc src/bin/main.rs
Segmentation fault: 11
$ RUST_BACKTRACE=1 rustc src/bin/main.rs
Illegal instruction: 4
