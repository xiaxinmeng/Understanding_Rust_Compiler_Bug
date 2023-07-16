
Compiling playground v0.0.1 (file:///playground)
error: internal compiler error: librustc/ty/subst.rs:493: Type parameter `TypeA/#0` (TypeA/0) out of range when substituting (root type=Some(GenericType<TypeA>)) substs=[]

thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:499:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: aborting due to previous error

note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.27.0-nightly (565235ee7 2018-05-07) running on x86_64-unknown-linux-gnu
note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin
note: some of the compiler flags provided by cargo are hidden
