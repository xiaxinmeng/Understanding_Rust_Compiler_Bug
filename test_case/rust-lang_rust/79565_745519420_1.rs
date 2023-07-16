`
$rustc -Zinstrument-coverage ./71348.rs
error: internal compiler error: compiler/rustc_symbol_mangling/src/v0.rs:537:17: symbol_names: unsupported constant of type `&str` (Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [105, 110, 116], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [7], len: Size { raw: 3 } }, size: Size { raw: 3 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 3 }) })

thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:958:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.50.0-nightly (e15ec667c 2020-12-15) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z instrument-coverage

query stack during panic:
#0 [symbol_name] computing the symbol for `<Foo as Get<"int">>::get`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: aborting due to previous error

