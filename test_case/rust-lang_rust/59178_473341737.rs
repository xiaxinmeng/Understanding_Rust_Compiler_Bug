
DEBUG 2019-03-15T15:53:54Z: rustc_typeck::check::writeback: write_ty_to_tables(HirId { owner: DefIndex(0:6), local_id: 6 }, fn() -> i32 {i32_identity<X : i32>})
thread 'rustc' panicked at 'assertion failed: !ty.needs_infer() && !ty.has_placeholders()', src/librustc_typeck/check/writeback.rs:119:9
