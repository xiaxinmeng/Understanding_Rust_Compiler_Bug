
% ./x86_64-apple-darwin/stage2/bin/rustc --version 
rustc 0.11.0-pre (2973ca6 2014-07-07 13:49:43 +1200)
host: x86_64-apple-darwin
% ./x86_64-apple-darwin/stage2/bin/rustc /tmp/g.rs && ./g
/tmp/g.rs:11:31: 11:36 error: internal compiler error: unsupported adjustment
/tmp/g.rs:11     let g : Box<Foo<[int]>> = box f;
                                           ^~~~~
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /Users/fklock/Dev/Mozilla/rust-dstnrc/src/libsyntax/diagnostic.rs:106


% 
