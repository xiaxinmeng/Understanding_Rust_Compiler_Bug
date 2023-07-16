
wtf.rs:11:13: 11:14 error: internal compiler error: coherence failed to report ambiguity: cannot locate the impl of the trait `core::kinds::Sized` for the type `&'a D`
wtf.rs:11         let r: &'a D = &self.d;
                      ^
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libsyntax/diagnostic.rs:123
