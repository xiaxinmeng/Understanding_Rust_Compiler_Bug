
thread 'rustc' panicked at 'Tried to access field 0 of union TyLayout {
    ty: CSeed,
    details: LayoutDetails {
        variants: Single {
            index: 0,
        },
        fields: Union(
            0,
        ),
        abi: Uninhabited,
        largest_niche: None,
        align: AbiAndPrefAlign {
            abi: Align {
                pow2: 0,
            },
            pref: Align {
                pow2: 0,
            },
        },
        size: Size {
            raw: 0,
        },
    },
} with 0 fields', src/librustc_mir/interpret/place.rs:413:17
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.44/src/backtrace/libunwind.rs:86
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.44/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:78
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1052
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1428
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:204
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:224
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:474
  12: rust_begin_unwind
             at src/libstd/panicking.rs:378
  13: std::panicking::begin_panic_fmt
             at src/libstd/panicking.rs:332
  14: rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::operand_field
  15: rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_place_to_op
  16: rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_operand
  17: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_rvalue_into_place
  18: <rustc_mir::transform::const_prop::ConstPropagator as rustc::mir::visit::MutVisitor>::visit_statement
  19: <rustc_mir::transform::const_prop::ConstProp as rustc_mir::transform::MirPass>::run_pass
  20: rustc_mir::transform::run_passes
  21: rustc_mir::transform::run_optimization_passes
  22: rustc_mir::transform::optimized_mir
  23: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::optimized_mir>::compute
  24: rustc::dep_graph::graph::DepGraph::with_task_impl
  25: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  26: rustc::ty::<impl rustc::ty::context::TyCtxt>::instance_mir
  27: rustc_mir::monomorphize::collector::collect_items_rec
  28: rustc_mir::monomorphize::collector::collect_items_rec
  29: rustc_mir::monomorphize::collector::collect_items_rec
  30: rustc_mir::monomorphize::collector::collect_items_rec
  31: rustc_session::utils::<impl rustc_session::session::Session>::time
  32: rustc_mir::monomorphize::collector::collect_crate_mono_items
  33: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
  34: rustc::ty::query::__query_compute::collect_and_partition_mono_items
  35: rustc::dep_graph::graph::DepGraph::with_task_impl
  36: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  37: rustc_codegen_ssa::base::codegen_crate
  38: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  39: rustc_session::utils::<impl rustc_session::session::Session>::time
  40: rustc_interface::passes::QueryContext::enter
  41: rustc_interface::queries::Queries::ongoing_codegen
  42: rustc_interface::interface::run_compiler_in_existing_thread_pool
  43: scoped_tls::ScopedKey<T>::set
  44: syntax::attr::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.43.0-nightly (5e7af4669 2020-02-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [optimized_mir] processing `<Composed as Machine>::create`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: could not compile `foo`.

To learn more, run the command again with --verbose.

