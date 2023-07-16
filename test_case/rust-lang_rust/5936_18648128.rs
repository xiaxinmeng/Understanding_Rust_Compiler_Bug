
jclements-09740:~/rust/build clements> RUST_LOG=rustc=1,::rt::backtrace !!
RUST_LOG=rustc=1,::rt::backtrace rustc /tmp/foo4.rs
Running /Users/clements/rust/build/x86_64-apple-darwin/stage2/bin/rustc:
rust: task failed at 'assertion failed: self.variance.is_some()', /Users/clements/rust/src/librustc/middle/typeck/rscope.rs:181
error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug
note: try running with RUST_LOG=rustc=1,::rt::backtrace to get further details and report the results to github.com/mozilla/rust/issues
rust: task failed at 'explicit failure', /Users/clements/rust/src/librustc/rustc.rc:368
rust: domain main @0x7ff95c019210 root task failed
