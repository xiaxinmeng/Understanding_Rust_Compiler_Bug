
cargo run --verbose
       Fresh winapi v0.2.7
       Fresh libc v0.2.11
       Fresh rustc-serialize v0.3.19
       Fresh bitflags v0.7.0
       Fresh winapi-build v0.1.1
       Fresh pkg-config v0.3.8
       Fresh linked-hash-map v0.0.9
       Fresh lru-cache v0.0.7
       Fresh kernel32-sys v0.2.2
   Compiling libsqlite3-sys v0.5.0
     Running `rustc C:\Users\james.tease\.cargo\registry\src\github.com-88ac128001ac3a9a\libsqlite3-sys-0.5.0\src\lib.rs --crate-name libsqlite3_sys --crate-type lib -g -C metadata=b72bcb022ad64b13 -C extra-filename=-b72bcb022ad64b13 --out-dir C:\Users\james.tease\Documents\todo\target\debug\deps --emit=dep-info,link -L dependency=C:\Users\james.tease\Documents\todo\target\debug\deps -L dependency=C:\Users\james.tease\Documents\todo\target\debug\deps --extern libc=C:\Users\james.tease\Documents\todo\target\debug\deps\liblibc-38919d24e617a235.rlib --cap-lints allow -L "\"C:\Program Files\sqlite3\"" -l sqlite3`
       Fresh time v0.1.35
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: JoinPathsError { inner: JoinPathsError }', ../src/libcore\result.rs:746
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: Could not compile `libsqlite3-sys`.

Caused by:
  Process didn't exit successfully: `rustc C:\Users\james.tease\.cargo\registry\src\github.com-88ac128001ac3a9a\libsqlite3-sys-0.5.0\src\lib.rs --crate-name libsqlite3_sys --crate-type lib -g -C metadata=b72bcb022ad64b13 -C extra-filename=-b72bcb022ad64b13 --out-dir C:\Users\james.tease\Documents\todo\target\debug\deps --emit=dep-info,link -L dependency=C:\Users\james.tease\Documents\todo\target\debug\deps -L dependency=C:\Users\james.tease\Documents\todo\target\debug\deps --extern libc=C:\Users\james.tease\Documents\todo\target\debug\deps\liblibc-38919d24e617a235.rlib --cap-lints allow -L "C:\Program Files\sqlite3" -l sqlite3` (exit code: 101)
