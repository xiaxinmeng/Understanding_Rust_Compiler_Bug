
<anon>:5:1: 9:2 error: internal compiler error: coherence failed to report ambiguity: cannot locate the impl of the trait `MyTrait` for the type `MyTrait + 'a`
<anon>:5 impl<'a> MyTrait for MyTrait + 'a {
<anon>:6     fn do_something(&self) {
<anon>:7         println!("MyTrait");
<anon>:8     }
<anon>:9 }
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libsyntax/diagnostic.rs:129
