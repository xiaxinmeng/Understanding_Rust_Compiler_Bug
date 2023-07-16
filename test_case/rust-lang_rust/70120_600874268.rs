
error: internal compiler error: src\librustc\traits\codegen\mod.rs:57: Encountered error `OutputTypeParameterMismatch(Binder(<[closure@.\70120.rs:26:17: 28:6] as std::ops::Fn<(<MyStruct as MyTrait<'_>>::Output,)>>), Binder(<[closure@.\70120.rs:26:17: 28:6] as std::ops::Fn<(&usize,)>>), Sorts(ExpectedFound { expected: &usize, found: <MyStruct as MyTrait<'_>>::Output }))` selecting `Binder(<[closure@.\70120.rs:26:17: 28:6] as std::ops::Fn<(&usize,)>>)` during codegen

thread 'rustc' panicked at 'Box<Any>', src\librustc_errors\lib.rs:873:9
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: <std::io::IoSliceMut as core::fmt::Debug>::fmt
   3: std::panicking::take_hook
   4: std::panicking::take_hook
   5: rustc_driver::report_ice
   6: std::panicking::rust_panic_with_hook
   7: <rustc_errors::snippet::Style as core::fmt::Debug>::fmt
   8: rustc_errors::HandlerInner::treat_err_as_bug
   9: rustc_errors::Handler::bug
  10: rustc::util::bug::bug_fmt
  11: <rustc::ty::trait_def::TraitImpls as rustc_data_structures::stable_hasher::HashStable<rustc::ich::hcx::StableHashingContext>>::hash_stable
  12: <rustc::ty::trait_def::TraitImpls as rustc_data_structures::stable_hasher::HashStable<rustc::ich::hcx::StableHashingContext>>::hash_stable
  13: rustc::util::bug::bug_fmt
  14: rustc::util::bug::bug_fmt
  15: rustc::ty::context::TyCtxt::caller_location_ty
  16: rustc::traits::codegen::codegen_fulfill_obligation
  17: <rustc::ty::query::Query as rustc_data_structures::stable_hasher::HashStable<rustc::ich::hcx::StableHashingContext>>::hash_stable
  18: rustc::middle::weak_lang_items::<impl rustc::ty::context::TyCtxt>::is_weak_lang_item
  19: rustc::dep_graph::graph::DepGraph::assert_ignored
  20: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::try_print_query_stack
  21: rustc::ty::instance::Instance::resolve
  22: <rustc_mir::monomorphize::collector::MirNeighborCollector as rustc::mir::visit::Visitor>::visit_terminator_kind
  23: rustc_mir::monomorphize::collector::collect_crate_mono_items
  24: rustc_mir::monomorphize::collector::collect_crate_mono_items
  25: <rustc_mir::util::elaborate_drops::Unwind as core::fmt::Debug>::fmt
  26: rustc_mir::monomorphize::collector::collect_crate_mono_items
  27: rustc_mir::monomorphize::partitioning::compute_codegen_unit_name
  28: <rustc_codegen_llvm::llvm_::archive_ro::ArchiveRO as core::ops::drop::Drop>::drop
  29: <rustc_codegen_llvm::llvm_::ffi::PassKind as core::fmt::Debug>::fmt
  30: <rustc_codegen_llvm::debuginfo::metadata::MemberDescription as core::fmt::Debug>::fmt
  31: <rustc_codegen_llvm::builder::Builder as rustc_codegen_ssa::traits::builder::BuilderMethods>::unchecked_umul
  32: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  33: <<serde::de::impls::range::Field as serde::de::Deserialize>::deserialize::FieldVisitor as serde::de::Visitor>::expecting
  34: rustc_interface::passes::BoxedResolver::to_resolver_outputs
  35: rustc_interface::queries::Queries::ongoing_codegen
  36: rustc_driver::pretty::print_after_hir_lowering
  37: <env_logger::filter::inner::Filter as core::fmt::Display>::fmt
  38: <env_logger::filter::inner::Filter as core::fmt::Display>::fmt
  39: <env_logger::filter::inner::Filter as core::fmt::Display>::fmt
  40: _rust_maybe_catch_panic
  41: rustc_driver::pretty::print_after_hir_lowering
  42: ZN244_$LT$std..error..$LT$impl$u20$core..convert..From$LT$alloc..string..String$GT$$u20$for$u20$alloc..boxed..Box$LT$dyn$u20$std..error..Error$u2b$core..marker..Sync$u2b$core..marker..Send$GT$$GT$..from..StringError$u20$as$u20$core..fmt..Display$GT$3fmt17
  43: std::sys::windows::thread::Thread::new
  44: BaseThreadInitThunk
  45: RtlUserThreadStart
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.42.0 (b8cedc004 2020-03-09) running on x86_64-pc-windows-msvc

query stack during panic:
#0 [codegen_fulfill_obligation] checking if `std::ops::Fn` fulfills its obligations
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: aborting due to previous error
