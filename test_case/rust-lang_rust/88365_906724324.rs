
$ mkdir ice
$ cd ice
$ cargo init --lib
$ cat << "EOF" > src/lib.rs
...your sample code...
EOF
$ rustup install nightly-2021-08-17
$ RUST_BACKTRACE=1 cargo +nightly-2021-08-17 test some_test
