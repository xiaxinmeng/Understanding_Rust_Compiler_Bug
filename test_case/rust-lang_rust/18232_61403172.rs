
<anon>:1:1: 1:1 error: internal compiler error: Type parameter out of range when substituting in region 'a (root type='aBar) (space=FnSpace, index=0)
<anon>:1 extern crate test;
         ^
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' panicked at 'Box<Any>', /build/rust-git/src/rust/src/libsyntax/diagnostic.rs:116
