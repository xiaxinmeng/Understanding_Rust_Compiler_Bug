
error: internal compiler error: compiler/rustc_mir/src/const_eval/mod.rs:160:14: cannot destructure constant Const { ty: &'static [&'static str], val: Value(ByRef { alloc: Allocation { bytes: [0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [(Size { raw: 0 }, ((), alloc4))] }), init_mask: InitMask { blocks: [65535], len: Size { raw: 16 } }, size: Size { raw: 16 }, align: Align { pow2: 3 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }

thread 'rustc' panicked at 'Box<Any>', /rustc/36f1f04f18b89ba4a999bcfd6584663fd6fc1c5d/library/std/src/panic.rs:59:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (36f1f04f1 2021-03-17) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [destructure_const] destructure constant
#1 [typeck] type-checking `main`
end of query stack
