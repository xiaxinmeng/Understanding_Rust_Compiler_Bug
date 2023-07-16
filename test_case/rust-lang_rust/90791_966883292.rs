
error: internal compiler error: compiler/rustc_codegen_llvm/src/context.rs:784:18: Unsupported target_c_int_width = 17
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1147:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.58.0-dev running on x86_64-unknown-linux-gnu
note: compiler flags: -Z unstable-options -C opt-level=s -C panic=abort -C linker-plugin-lto -C codegen-units=1 -C debuginfo=2 --crate-type lib
note: some of the compiler flags provided by cargo are hidden
