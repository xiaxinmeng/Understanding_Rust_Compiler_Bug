
$ rustup run nightly rustc -vV
rustc 1.13.0-nightly (0be88eb79 2016-09-11)
binary: rustc
commit-hash: 0be88eb794949d27331ec45c300a40369b541001
commit-date: 2016-09-11
host: x86_64-unknown-linux-gnu
release: 1.13.0-nightly
$ rustup run nightly rustc -C target-cpu=native test.rs
$ ./test
Ok("my/current/dir")
