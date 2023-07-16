 bash
$ cargo build
   Compiling rust-GSL v0.4.9 (file:///home/mdinger/Programs/gsl/rust-GSL)
src/rgsl.rs:23:5: 23:16 error: internal compiler error: &mut fn(f64, &mut T) -> f64 was a subtype of &mut fn(<generic #15>, <generic #16>) -> <generic #14> but now is not?
src/rgsl.rs:23     f(x, p.arg) * sinwx
                   ^~~~~~~~~~~
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libsyntax/ast_util.rs:776


Could not compile `rust-GSL`.

To learn more, run the command again with --verbose.
