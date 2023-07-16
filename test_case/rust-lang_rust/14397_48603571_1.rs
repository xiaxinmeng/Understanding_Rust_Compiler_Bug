
% ./x86_64-apple-darwin/stage2/bin/rustc --cfg buggy /tmp/h.rs && ./h
error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'get_base_type() returned a type that wasn't an enum, struct, or trait', /Users/fklock/Dev/Mozilla/rust-dstnrc/src/librustc/middle/typeck/coherence.rs:169
