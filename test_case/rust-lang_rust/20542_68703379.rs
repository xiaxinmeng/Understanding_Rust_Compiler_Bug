
/home/jonas/rust/linalg/src/vector.rs:7:13: 7:19 error: default type parameters are experimental and possibly buggy [E0108]
/home/jonas/rust/linalg/src/vector.rs:7 impl <U, T: Add<U>> Add for Vector2<T> {
                                                    ^~~~~~
/home/jonas/rust/linalg/src/vector.rs:7:13: 7:19 help: add #![feature(default_type_params)] to the crate attributes to enable
/home/jonas/rust/linalg/src/vector.rs:7 impl <U, T: Add<U>> Add for Vector2<T> {
                                                    ^~~~~~
ERROR:rbml::reader: failed to find block with tag 7
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
fish: Job 1, “env RUST_BACKTRACE=1 rustc /home/jonas/rust/linalg/src/lib.rs --crate-name linalg --crate-type lib -g -C metadata=467966ee376b8dd3 -C extra-filename=-467966ee376b8dd3 --out-dir /home/jonas/rust/linalg/target --emit=dep-info,link -L /home/jonas/rust/linalg/target -L /home/jonas/rust/linalg/target/deps” terminated by signal SIGILL (Illegal instruction)
