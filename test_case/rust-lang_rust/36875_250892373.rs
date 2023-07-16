
$ cargo init repro
$ cd repro
$ echo 'pub fn f() { let _: Vec<char> = "x".chars().collect(); }' > src/lib.rs
$ rustup run 1.12.0 cargo build --release --verbose
