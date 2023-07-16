
thread 'rustc' panicked at 'internal error: entered unreachable code: FieldsShape::offset: `Primitive`s have no fields', /rustc/9bb77da74dac4768489127d21e32db19b59ada5b/compiler/rustc_target/src/abi/mod.rs:895:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (9bb77da74 2021-09-13) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z print-type-sizes -C prefer-dynamic -C embed-bitcode=no -C debug-assertions=off --crate-type proc-macro

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [layout_of] computing layout of `std::result::Result<std::convert::Infallible, !>`
#1 [layout_of] computing layout of `std::ops::ControlFlow<std::result::Result<std::convert::Infallible, !>>`
end of query stack
error: could not compile `serde_repr`
