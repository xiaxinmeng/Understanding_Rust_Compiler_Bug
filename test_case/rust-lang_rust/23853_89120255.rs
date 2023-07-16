
src/middleware.rs:38:1: 40:2 error: internal compiler error: coherence failed to report ambiguity: cannot locate the impl of the trait `core::marker::MarkerTrait` for the type `handlebars::helpers::HelperDef`
src/middleware.rs:38 impl typemap::Key for HandlebarsEngine {
src/middleware.rs:39     type Value = Template;
src/middleware.rs:40 }
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libsyntax/diagnostic.rs:130
