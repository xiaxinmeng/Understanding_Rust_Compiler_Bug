
/tmp $ rustc hi.rs
hi.rs:5:16: 5:19 error: internal compiler error: Type parameter `Bar/TypeSpace.0` (Bar/TypeSpace/0) out of range when substituting (root type=Bar) substs=Substs[types=[[];[];[]], regions=[[];[];[]]]
hi.rs:5 pub struct Baz(Foo);
                       ^~~
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/libsyntax/diagnostic.rs:129
