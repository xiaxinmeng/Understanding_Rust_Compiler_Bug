
<anon>:7:12: 7:27 error: the type of this value must be known in this context
<anon>:7     AnyFoo(box Foobar as _);
                    ^~~~~~~~~~~~~~~
error: internal compiler error: asked to compute contents of error type
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' panicked at 'Box<Any>', /build/rust-git/src/rust/src/libsyntax/diagnostic.rs:180


playpen: application terminated with error code 101
