
landin:~ lkuper$ rustc foo.rs
warning: no debug symbols in executable (-arch x86_64)
landin:~ lkuper$ export RUST_LOG=foo; ./foo
rust: ~"error from module a"
rust: ~"warn from module a"
rust: ~"info from module a"
rust: ~"debug from module a"
rust: ~"error from module b"
rust: ~"warn from module b"
rust: ~"info from module b"
rust: ~"debug from module b"
landin:~ lkuper$
