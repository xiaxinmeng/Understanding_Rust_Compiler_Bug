
thread 'rustc' panicked at 'internal error: entered unreachable code: FieldsShape::offset: `Primitive`s have no fields', compiler/rustc_target/src/abi/mod.rs:781:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0-nightly (af140757b 2021-08-22) running on x86_64-apple-darwin

note: compiler flags: -Z print-type-sizes -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
warning: `vulkano` (lib) generated 2 warnings
error: could not compile `vulkano`; 2 warnings emitted

