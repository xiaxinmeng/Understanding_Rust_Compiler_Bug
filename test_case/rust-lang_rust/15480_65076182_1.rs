
<anon>:4:10: 4:12 error: internal compiler error: Explicit deref of non-derefable type: core::slice::Items<'_, &int>
<anon>:4     for &&x in v.iter() {
                  ^~
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' panicked at 'Box<Any>', /build/rust-git/src/rust/src/libsyntax/diagnostic.rs:116

playpen: application terminated with error code 101
Program ended.
