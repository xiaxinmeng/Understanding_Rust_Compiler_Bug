
error: internal compiler error: compiler/rustc_trait_selection/src/traits/codegen.rs:78:17: Encountered error `OutputTypeParameterMismatch(Binder(<[closure@src/main.rs:43:19: 43:50] as std::ops::FnMut<(<Windows<i32> as Iter>::Item<'_>,)>>), Binder(<[closure@src/main.rs:43:19: 43:50] as std::ops::FnMut<(&mut [i32],)>>), Sorts(ExpectedFound { expected: &mut [i32], found: <Windows<i32> as Iter>::Item<'_> }))` selecting `Binder(<[closure@src/main.rs:43:19: 43:50] as std::ops::FnMut<(&mut [i32],)>>)` during codegen

thread 'rustc' panicked at 'Box<Any>', /rustc/3f5aee2d5241139d808f4fdece0026603489afd1/library/std/src/panic.rs:59:5
stack backtrace:
   0: std::panicking::begin_panic
   1: std::panic::panic_any
   2: rustc_errors::HandlerInner::bug
   3: rustc_errors::Handler::bug
   4: rustc_middle::ty::context::tls::with_opt
   5: rustc_middle::util::bug::opt_span_bug_fmt
   6: rustc_middle::util::bug::bug_fmt
   7: rustc_infer::infer::InferCtxtBuilder::enter
   8: rustc_trait_selection::traits::codegen::codegen_fulfill_obligation
   9: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::codegen_fulfill_obligation>::compute
  10: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  11: rustc_data_structures::stack::ensure_sufficient_stack
  12: rustc_query_system::query::plumbing::force_query_with_job
  13: rustc_query_system::query::plumbing::get_query_impl
  14: rustc_ty_utils::instance::inner_resolve_instance
  15: rustc_ty_utils::instance::resolve_instance
  16: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::resolve_instance>::compute
  17: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  18: rustc_data_structures::stack::ensure_sufficient_stack
  19: rustc_query_system::query::plumbing::force_query_with_job
  20: rustc_query_system::query::plumbing::get_query_impl
  21: rustc_middle::ty::instance::Instance::resolve_opt_const_arg
  22: rustc_middle::ty::instance::Instance::resolve
  23: <rustc_mir::monomorphize::collector::MirNeighborCollector as rustc_middle::mir::visit::Visitor>::visit_terminator
  24: rustc_mir::monomorphize::collector::collect_neighbours
  25: rustc_mir::monomorphize::collector::collect_items_rec
  26: rustc_mir::monomorphize::collector::collect_items_rec
  27: rustc_session::utils::<impl rustc_session::session::Session>::time
  28: rustc_mir::monomorphize::collector::collect_crate_mono_items
  29: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
  30: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::collect_and_partition_mono_items>::compute
  31: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  32: rustc_data_structures::stack::ensure_sufficient_stack
  33: rustc_query_system::query::plumbing::force_query_with_job
  34: rustc_query_system::query::plumbing::get_query_impl
  35: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
  36: rustc_session::utils::<impl rustc_session::session::Session>::time
  37: rustc_interface::queries::Queries::ongoing_codegen
  38: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  39: rustc_span::with_source_map
  40: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (3f5aee2d5 2021-02-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [codegen_fulfill_obligation] checking if `std::ops::FnMut` fulfills its obligations
#1 [resolve_instance] resolving instance `<[closure@src/main.rs:43:19: 43:50] as std::ops::FnMut<(&mut [i32],)>>::call_mut`
#2 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
