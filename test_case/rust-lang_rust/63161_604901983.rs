
Running `rustc --edition=2018 --crate-name blocks src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --emit=dep-info,link -C debuginfo=2 --test -C metadata=3630b688124f0f8c -C extra-filename=-3630b688124f0f8c --out-dir [...]/target/debug/deps -C incremental=[...]/target/debug/incremental -L dependency=[...]/target/debug/deps`
thread 'rustc' panicked at 'index out of bounds: the len is 115 but the index is 116', /builddir/rustc-1.41.1-src/src/libcore/slice/mod.rs:2806:10
stack backtrace:
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: <unknown>
   3: <unknown>
   4: <unknown>
   5: rustc_driver::report_ice
   6: <unknown>
   7: std::panicking::rust_panic_with_hook
   8: rust_begin_unwind
   9: core::panicking::panic_fmt
  10: core::panicking::panic_bounds_check
  11: <rustc::ty::query::on_disk_cache::CacheDecoder as serialize::serialize::SpecializedDecoder<syntax_pos::span_encoding::Span>>::specialized_decode
  12: <unknown>
  13: <unknown>
  14: <unknown>
  15: <unknown>
  16: <unknown>
  17: <unknown>
  18: <unknown>
  19: <unknown>
  20: <unknown>
  21: rustc::ty::<impl rustc::ty::context::TyCtxt>::instance_mir
  22: <unknown>
  23: <unknown>
  24: <unknown>
  25: rustc_mir::monomorphize::collector::collect_crate_mono_items
  26: <unknown>
  27: <unknown>
  28: <unknown>
  29: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  30: <unknown>
  31: rustc_interface::queries::Queries::ongoing_codegen
  32: <unknown>
  33: <unknown>
  34: __rust_maybe_catch_panic
  35: <unknown>
  36: rust_metadata_std_3c2b88695d4d9829146c62c7533daf14
  37: <unknown>
  38: start_thread
             at /builddir/glibc-2.30/nptl/pthread_create.c:479
  39: __clone
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.41.1 running on powerpc64le-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [optimized_mir] processing `test::find::{{closure}}#0`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
