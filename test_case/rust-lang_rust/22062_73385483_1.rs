
<anon>:50:9: 50:21 warning: unused variable: `foocontainer`, #[warn(unused_variables)] on by default
<anon>:50     let foocontainer: FooContainer<F> = FooContainer { foo: Foo { member: ptr::null_mut() } };
                  ^~~~~~~~~~~~
error: internal compiler error: type_of with ty_err
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libsyntax/diagnostic.rs:189
