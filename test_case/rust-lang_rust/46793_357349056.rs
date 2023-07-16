
Compiling openssl v0.9.23
error: internal compiler error: /checkout/src/librustc/ty/subst.rs:424: Region parameter out of range when substituting in region 'a (root type=Some(unsafe fn(*mut <Self as foreign_types::ForeignTypeRef>::CType) -> &'a mut Self {<Self as foreign_types::ForeignTypeRef>::from_ptr_mut::<'a>})) (index=1)
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.24.0-beta.2 (a19122cfa 2018-01-10) running on x86_64-unknown-linux-gnu
thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:451:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Could not compile `openssl`.
