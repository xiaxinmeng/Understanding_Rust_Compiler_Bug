
/Users/b.shaw/Projects/rust/bug.rs:4:5: 4:15 error: internal compiler error: &mut [f64] was a subtype of &mut <generic #38> but now is not?
/Users/b.shaw/Projects/rust/bug.rs:4     args.sum()
                                             ^~~~~~~~~~
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-mac/build/src/libsyntax/diagnostic.rs:116
