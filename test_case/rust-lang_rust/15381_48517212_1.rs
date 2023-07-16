
rust_error.rs:13:13: 13:20 error: internal compiler error: Explicit index of non-index type `&Matrix`
rust_error.rs:13     let e = m[0][0];
                             ^~~~~~~
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', C:\bot\slave\nightly-win\build\src\libsyntax\diagnostic.rs:107
