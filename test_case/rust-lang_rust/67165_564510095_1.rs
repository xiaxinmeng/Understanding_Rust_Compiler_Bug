
$ RUST_BACKTRACE=1 cargo test
   Compiling day11 v0.1.0 (.../aoc/2019/day11)
thread 'rustc' panicked at 'index out of bounds: the len is 178 but the index is 178', /rustc/4560ea788cb760f0a34127156c78e2552949f734/src/libcore/slice/mod.rs:2717:1
0
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:76
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:60
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1030
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1412
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:64
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:196
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:210
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:477
  12: std::panicking::continue_panic_fmt
             at src/libstd/panicking.rs:380
  13: rust_begin_unwind
             at src/libstd/panicking.rs:307
  14: core::panicking::panic_fmt
             at src/libcore/panicking.rs:85
  15: core::panicking::panic_bounds_check
             at src/libcore/panicking.rs:61
  16: <rustc::ty::query::on_disk_cache::CacheDecoder as serialize::serialize::SpecializedDecoder<syntax_pos::span_encoding::Span>>::specialized_decode
  17: serialize::serialize::Decoder::read_struct
  18: serialize::serialize::Decoder::read_seq
  19: serialize::serialize::Decoder::read_struct
  20: serialize::serialize::Decoder::read_seq
  21: <rustc::mir::Body as serialize::serialize::Decodable>::decode::{{closure}}
  22: rustc::ty::query::on_disk_cache::OnDiskCache::try_load_query_result
  23: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  24: rustc_mir::monomorphize::collector::collect_items_rec
  25: rustc_mir::monomorphize::collector::collect_items_rec
  26: rustc_mir::monomorphize::collector::collect_crate_mono_items::{{closure}}
  27: rustc_mir::monomorphize::collector::collect_crate_mono_items
  28: rustc::util::common::time
  29: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
  30: rustc::ty::query::__query_compute::collect_and_partition_mono_items
  31: rustc::dep_graph::graph::DepGraph::with_task_impl
  32: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  33: rustc_codegen_ssa::base::codegen_crate
  34: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  35: rustc::util::common::time
  36: rustc_interface::passes::start_codegen
  37: rustc::ty::context::tls::enter_global
  38: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  39: rustc_interface::passes::create_global_ctxt::{{closure}}
  40: rustc_interface::passes::BoxedGlobalCtxt::enter
  41: rustc_interface::queries::Query<T>::compute
  42: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::ongoing_codegen
  43: rustc_interface::interface::run_compiler_in_existing_thread_pool
  44: std::thread::local::LocalKey<T>::with
  45: scoped_tls::ScopedKey<T>::set
  46: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.39.0 (4560ea788 2019-11-04) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [optimized_mir] processing `full_stage2::{{closure}}#0`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: could not compile `day11`.

To learn more, run the command again with --verbose.
