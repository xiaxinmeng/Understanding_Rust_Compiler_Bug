
$ RUST_BACKTRACE=1 build/x86_64-unknown-linux-gnu/stage1/bin/rustc -Aunused -g a.rs
thread '<unnamed>' panicked at 'help', compiler/rustc_driver/src/lib.rs:1333:5
stack backtrace:
[1]    3509283 segmentation fault (core dumped)  RUST_BACKTRACE=1 build/x86_64-unknown-linux-gnu/stage1/bin/rustc -Aunused -g
