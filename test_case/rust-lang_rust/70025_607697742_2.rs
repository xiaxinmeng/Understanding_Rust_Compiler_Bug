
$ rustup-toolchain-install-master 40f2b445cd1c6c4e72d663d23c6d1bef2c674ad4 -c rust-src
$ cargo +40f2b445cd1c6c4e72d663d23c6d1bef2c674ad4 doc
 Documenting test-70025 v0.1.0 (/home/eddy/Projects/test-70025)
    Finished dev [unoptimized + debuginfo] target(s) in 2.21s
$ find target/doc/src/ | rg html
target/doc/src/test_70025/lib.rs.html
target/doc/src/test_70025/home/eddy/.rustup/toolchains/40f2b445cd1c6c4e72d663d23c6d1bef2c674ad4/lib/rustlib/src/rust/src/libstd/thread/local.rs.html
