
$ RUST_LOG=rustc=1,::rt::backtrace rustc enum-match-ice.rs
rust: task failed at 'Non-constant path in constant expr', /-------/rust/src/librustc/middle/const_eval.rs:245
error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug
note: try running with RUST_LOG=rustc=1,::rt::backtrace to get further details and report the results to github.com/mozilla/rust/issues
rust: task failed at 'explicit failure', /------/rust/src/librustc/rustc.rc:355
rust: domain main @0x7f8b7080ba10 root task failed
