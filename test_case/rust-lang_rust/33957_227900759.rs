
src/basename_for_chop.rs:21:12: 21:26 error: internal compiler error: ../src/librustc_trans/debuginfo/metadata.rs:909: debuginfo: Could not find scope info for node NodeExpr(expr(3918: '/'))
src/basename_for_chop.rs:21       Some(MAIN_SEPARATOR) => { offset = offset + 1 },
                                       ^~~~~~~~~~~~~~
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'Box<Any>', ../src/libsyntax/errors/mod.rs:545
