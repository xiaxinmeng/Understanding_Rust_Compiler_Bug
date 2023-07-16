
[mw@localhost ice-mono-item]$ RUST_BACKTRACE=1 cargo check
   Compiling proc-macro2 v0.4.27
   Compiling unicode-xid v0.1.0
   Compiling cc v1.0.29
   Compiling libc v0.2.48
   Compiling autocfg v0.1.2
   Compiling failure_derive v0.1.5
   Compiling cfg-if v0.1.6
   Compiling cortex-m v0.3.1
   Compiling rustc-demangle v0.1.13
    Checking vcell v0.1.0
    Checking aligned v0.1.2
    Checking bare-metal v0.1.3
   Compiling cortex-m v0.4.3
   Compiling cortex-m-rt v0.4.0
    Checking void v1.0.2
   Compiling either v1.5.0
    Checking r0 v0.2.2
    Checking nb v0.1.1
    Checking untagged-option v0.1.1
   Compiling typenum v1.10.0
   Compiling cortex-m-rtfm v0.3.4
   Compiling fpa v0.1.0
   Compiling byteorder v1.3.1
    Checking cast v0.2.2
   Compiling nrf51-ble-demo v0.1.0 (/home/mw/stuff/ice-mono-item)
    Checking rtfm-core v0.2.0
    Checking panic-halt v0.2.0
    Checking volatile-register v0.2.0
    Checking embedded-hal v0.2.2
    Checking embedded-hal v0.1.3
   Compiling backtrace v0.3.13
error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:745: Cannot create local mono-item for DefId(9/0:10 ~ r0[590c]::zero_bss[0])

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:620:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:70
             at src/libstd/sys_common/backtrace.rs:58
   2: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:200
   3: std::panicking::default_hook
             at src/libstd/panicking.rs:215
   4: rustc::util::common::panic_hook
             at src/librustc/util/common.rs:39
   5: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:482
   6: std::panicking::begin_panic
             at /home/mw/1-rust/src/libstd/panicking.rs:412
   7: rustc_errors::Handler::bug
             at src/librustc_errors/lib.rs:620
   8: rustc::util::bug::opt_span_bug_fmt::{{closure}}
             at src/librustc/util/bug.rs:36
   9: rustc::ty::context::tls::with_opt::{{closure}}
             at src/librustc/ty/context.rs:2111
  10: rustc::ty::context::tls::with_context_opt
             at src/librustc/ty/context.rs:2046
  11: rustc::ty::context::tls::with_opt
             at src/librustc/ty/context.rs:2111
  12: rustc::util::bug::opt_span_bug_fmt
             at src/librustc/util/bug.rs:32
  13: rustc::util::bug::bug_fmt
             at src/librustc/util/bug.rs:12
  14: rustc_mir::monomorphize::collector::should_monomorphize_locally
             at src/librustc_mir/monomorphize/collector.rs:745
  15: rustc_mir::monomorphize::collector::visit_instance_use
             at src/librustc_mir/monomorphize/collector.rs:681
  16: <rustc_mir::monomorphize::collector::MirNeighborCollector<'a, 'tcx> as rustc::mir::visit::Visitor<'tcx>>::visit_terminator_kind
             at src/librustc_mir/monomorphize/collector.rs:0
  17: rustc::mir::visit::Visitor::visit_mir
             at /home/mw/1-rust/src/librustc/mir/visit.rs:444
             at /home/mw/1-rust/src/librustc/mir/visit.rs:109
             at /home/mw/1-rust/src/librustc/mir/visit.rs:343
             at /home/mw/1-rust/src/librustc/mir/visit.rs:82
             at /home/mw/1-rust/src/librustc/mir/visit.rs:295
             at /home/mw/1-rust/src/librustc/mir/visit.rs:76
  18: rustc_mir::monomorphize::collector::collect_items_rec
             at src/librustc_mir/monomorphize/collector.rs:1185
             at src/librustc_mir/monomorphize/collector.rs:395
  19: rustc_mir::monomorphize::collector::collect_items_rec
             at src/librustc_mir/monomorphize/collector.rs:405
  20: rustc_mir::monomorphize::collector::collect_crate_mono_items::{{closure}}
             at src/librustc_mir/monomorphize/collector.rs:303
             at /home/mw/1-rust/src/libcore/iter/traits/iterator.rs:604
             at /home/mw/1-rust/src/libcore/iter/traits/iterator.rs:1684
             at /home/mw/1-rust/src/libcore/iter/traits/iterator.rs:1572
             at /home/mw/1-rust/src/libcore/iter/traits/iterator.rs:1684
             at /home/mw/1-rust/src/libcore/iter/traits/iterator.rs:604
             at src/librustc_mir/monomorphize/collector.rs:301
  21: rustc::util::common::time
             at /home/mw/1-rust/src/librustc/util/common.rs:150
             at /home/mw/1-rust/src/librustc/util/common.rs:144
  22: rustc_mir::monomorphize::collector::collect_crate_mono_items
             at src/librustc_mir/monomorphize/collector.rs:300
  23: rustc::util::common::time
             at src/librustc_mir/monomorphize/partitioning.rs:927
             at /home/mw/1-rust/src/librustc/util/common.rs:150
             at /home/mw/1-rust/src/librustc/util/common.rs:144
  24: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
             at src/librustc_mir/monomorphize/partitioning.rs:926
  25: rustc::ty::query::__query_compute::collect_and_partition_mono_items
             at /home/mw/1-rust/src/librustc/ty/query/plumbing.rs:963
             at /home/mw/1-rust/src/librustc/ty/query/plumbing.rs:922
  26: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::collect_and_partition_mono_items<'tcx>>::compute
             at /home/mw/1-rust/src/librustc/ty/query/plumbing.rs:955
  27: rustc::dep_graph::graph::DepGraph::with_task_impl
             at /home/mw/1-rust/src/librustc/dep_graph/graph.rs:334
  28: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
             at /home/mw/1-rust/src/librustc/dep_graph/graph.rs:202
             at /home/mw/1-rust/src/librustc/ty/query/plumbing.rs:567
             at /home/mw/1-rust/src/librustc/ty/query/plumbing.rs:280
             at /home/mw/1-rust/src/librustc/ty/context.rs:1966
             at /home/mw/1-rust/src/librustc/ty/context.rs:1899
             at /home/mw/1-rust/src/librustc/ty/context.rs:1965
             at /home/mw/1-rust/src/librustc/ty/query/plumbing.rs:279
             at /home/mw/1-rust/src/librustc/ty/context.rs:2072
             at /home/mw/1-rust/src/librustc/ty/context.rs:2056
             at /home/mw/1-rust/src/librustc/ty/context.rs:2046
             at /home/mw/1-rust/src/librustc/ty/context.rs:2056
             at /home/mw/1-rust/src/librustc/ty/context.rs:2068
             at /home/mw/1-rust/src/librustc/ty/query/plumbing.rs:268
             at /home/mw/1-rust/src/librustc/ty/query/plumbing.rs:559
             at /home/mw/1-rust/src/librustc/ty/query/plumbing.rs:214
             at /home/mw/1-rust/src/librustc/ty/query/plumbing.rs:558
             at /home/mw/1-rust/src/librustc/ty/query/plumbing.rs:386
  29: core::ops::function::FnOnce::call_once
             at /home/mw/1-rust/src/librustc/ty/query/plumbing.rs:1040
             at /home/mw/1-rust/src/librustc/ty/query/plumbing.rs:1032
             at src/librustc_codegen_ssa/base.rs:919
             at /home/mw/1-rust/src/libcore/ops/function.rs:231
  30: rustc::ty::query::__query_compute::backend_optimization_level
             at /home/mw/1-rust/src/librustc/ty/query/plumbing.rs:963
             at /home/mw/1-rust/src/librustc/ty/query/plumbing.rs:922
  31: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::backend_optimization_level<'tcx>>::compute
             at /home/mw/1-rust/src/librustc/ty/query/plumbing.rs:955
  32: rustc::dep_graph::graph::DepGraph::with_task_impl
             at /home/mw/1-rust/src/librustc/dep_graph/graph.rs:334
  33: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
             at /home/mw/1-rust/src/librustc/dep_graph/graph.rs:202
             at /home/mw/1-rust/src/librustc/ty/query/plumbing.rs:567
             at /home/mw/1-rust/src/librustc/ty/query/plumbing.rs:280
             at /home/mw/1-rust/src/librustc/ty/context.rs:1966
             at /home/mw/1-rust/src/librustc/ty/context.rs:1899
             at /home/mw/1-rust/src/librustc/ty/context.rs:1965
             at /home/mw/1-rust/src/librustc/ty/query/plumbing.rs:279
             at /home/mw/1-rust/src/librustc/ty/context.rs:2072
             at /home/mw/1-rust/src/librustc/ty/context.rs:2056
             at /home/mw/1-rust/src/librustc/ty/context.rs:2046
             at /home/mw/1-rust/src/librustc/ty/context.rs:2056
             at /home/mw/1-rust/src/librustc/ty/context.rs:2068
             at /home/mw/1-rust/src/librustc/ty/query/plumbing.rs:268
             at /home/mw/1-rust/src/librustc/ty/query/plumbing.rs:559
             at /home/mw/1-rust/src/librustc/ty/query/plumbing.rs:214
             at /home/mw/1-rust/src/librustc/ty/query/plumbing.rs:558
             at /home/mw/1-rust/src/librustc/ty/query/plumbing.rs:386
  34: rustc_codegen_ssa::back::write::start_async_codegen
             at /home/mw/1-rust/src/librustc/ty/query/plumbing.rs:1040
             at /home/mw/1-rust/src/librustc/ty/query/plumbing.rs:1032
             at /home/mw/1-rust/src/librustc_codegen_ssa/back/write.rs:1025
             at /home/mw/1-rust/src/librustc_codegen_ssa/back/write.rs:441
  35: rustc_codegen_ssa::base::codegen_crate
             at /home/mw/1-rust/src/librustc_codegen_ssa/base.rs:575
  36: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
             at src/librustc_codegen_llvm/lib.rs:301
  37: rustc::util::common::time
             at src/librustc_driver/driver.rs:1354
             at /home/mw/1-rust/src/librustc/util/common.rs:150
             at /home/mw/1-rust/src/librustc/util/common.rs:144
  38: rustc_driver::driver::phase_4_codegen
             at src/librustc_driver/driver.rs:1354
  39: rustc_driver::driver::compile_input::{{closure}}
             at src/librustc_driver/driver.rs:316
  40: <std::thread::local::LocalKey<T>>::with
             at src/librustc_driver/driver.rs:1337
             at /home/mw/1-rust/src/librustc/ty/context.rs:2000
             at /home/mw/1-rust/src/librustc/ty/context.rs:1966
             at /home/mw/1-rust/src/librustc/ty/context.rs:1899
             at /home/mw/1-rust/src/librustc/ty/context.rs:1965
             at /home/mw/1-rust/src/librustc/ty/context.rs:1999
             at /home/mw/1-rust/src/librustc/ty/context.rs:1954
             at /home/mw/1-rust/src/libstd/thread/local.rs:300
             at /home/mw/1-rust/src/libstd/thread/local.rs:246
             at /home/mw/1-rust/src/librustc/ty/context.rs:1946
             at /home/mw/1-rust/src/libstd/thread/local.rs:300
             at /home/mw/1-rust/src/libstd/thread/local.rs:246
  41: rustc::ty::context::TyCtxt::create_and_enter
             at /home/mw/1-rust/src/librustc/ty/context.rs:1938
             at /home/mw/1-rust/src/librustc/ty/context.rs:1977
             at /home/mw/1-rust/src/librustc/ty/context.rs:1290
  42: rustc_driver::driver::phase_3_run_analysis_passes
             at src/librustc_driver/driver.rs:1213
  43: rustc_driver::driver::compile_input
             at src/librustc_driver/driver.rs:272
  44: rustc_driver::run_compiler_with_pool
             at src/librustc_driver/lib.rs:523
  45: <scoped_tls::ScopedKey<T>>::set
             at src/librustc_driver/lib.rs:445
             at src/librustc_driver/driver.rs:65
             at /home/mw/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
  46: rustc_driver::run_compiler
             at src/librustc_driver/driver.rs:64
             at src/librustc_driver/lib.rs:444
  47: <scoped_tls::ScopedKey<T>>::set
             at src/librustc_driver/lib.rs:1646
             at src/librustc_driver/lib.rs:167
             at /home/mw/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
             at /home/mw/1-rust/src/libsyntax/lib.rs:100
             at /home/mw/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
#1 [backend_optimization_level] optimization level used by backend
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -C opt-level=s -C debuginfo=2 -C debug-assertions=on -C link-arg=-Tlink.x --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `cortex-m-rt`.
warning: build failed, waiting for other jobs to finish...
error: build failed
