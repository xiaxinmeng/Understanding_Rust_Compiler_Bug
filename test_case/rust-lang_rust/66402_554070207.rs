
error: internal compiler error: src\librustc_traits\normalize_erasing_regions.rs:42: could not fully normalize `unsafe extern "system" fn(*mut rt::gen::windows::foundation::AsyncActionProgressHandler<TProgress>, <rt::gen::windows::foundation::IAsyncActionWithProgress<TProgress> as rt::RtType>::Abi, <TProgress as rt::RtType>::Abi) -> i32`

thread 'rustc' panicked at 'Box<Any>', src\librustc_errors\lib.rs:937:9
stack backtrace:
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: <std::io::IoSliceMut as core::fmt::Debug>::fmt
   3: std::panicking::take_hook
   4: std::panicking::take_hook
   5: rustc_driver::report_ice
   6: std::panicking::rust_panic_with_hook
   7: <rustc_errors::emitter::FileWithAnnotatedLines as core::fmt::Debug>::fmt
   8: rustc_errors::HandlerInner::abort_if_errors_and_should_abort
   9: rustc_errors::Handler::bug
  10: rustc::util::bug::bug_fmt
  11: <rustc::ty::trait_def::TraitImpls as rustc_data_structures::stable_hasher::HashStable<rustc::ich::hcx::StableHashingContext>>::hash_stable
  12: <rustc::ty::trait_def::TraitImpls as rustc_data_structures::stable_hasher::HashStable<rustc::ich::hcx::StableHashingContext>>::hash_stable
  13: <rustc::ty::trait_def::TraitImpls as rustc_data_structures::stable_hasher::HashStable<rustc::ich::hcx::StableHashingContext>>::hash_stable
  14: rustc::util::bug::bug_fmt
  15: rustc::util::bug::bug_fmt
  16: <rustc::ty::Predicate as rustc_traits::lowering::Lower<rustc::ty::sty::Binder<rustc::traits::DomainGoal>>>::lower
  17: <rustc::ty::Predicate as rustc_traits::lowering::Lower<rustc::ty::sty::Binder<rustc::traits::DomainGoal>>>::lower
  18: rustc::ty::print::pretty::<impl core::fmt::Display for rustc::ty::subst::GenericArg>::fmt
  19: rustc::middle::weak_lang_items::<impl rustc::ty::context::TyCtxt>::is_weak_lang_item
  20: rustc::dep_graph::graph::DepGraph::assert_ignored
  21: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::try_print_query_stack
  22: <rustc::traits::query::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc::ty::fold::TypeFolder>::fold_ty
  23: <rustc_lint::types::TypeLimits as rustc::lint::LateLintPass>::check_expr
  24: <rustc_lint::types::TypeLimits as rustc::lint::LateLintPass>::check_expr
  25: <rustc_lint::types::TypeLimits as rustc::lint::LateLintPass>::check_expr
  26: <rustc_lint::types::TypeLimits as rustc::lint::LateLintPass>::check_expr
  27: <rustc_lint::types::TypeLimits as rustc::lint::LateLintPass>::check_expr
  28: <rustc_lint::types::TypeLimits as rustc::lint::LateLintPass>::check_expr
  29: <rustc_lint::types::TypeLimits as rustc::lint::LateLintPass>::check_expr
  30: <rustc_lint::types::TypeLimits as rustc::lint::LateLintPass>::check_expr
  31: <rustc_lint::types::ImproperCTypesVisitor::check_for_opaque_ty::ProhibitOpaqueTypes as rustc::ty::fold::TypeVisitor>::visit_ty
  32: <rustc_lint::types::ImproperCTypesVisitor::check_for_opaque_ty::ProhibitOpaqueTypes as rustc::ty::fold::TypeVisitor>::visit_ty
  33: <rustc_lint::BuiltinCombinedModuleLateLintPass as rustc::lint::LateLintPass>::check_struct_field
  34: <rustc_lint::types::VariantSizeDifferences as rustc::lint::LintPass>::name
  35: <rustc_lint::BuiltinCombinedModuleLateLintPass as rustc::lint::LateLintPass>::check_struct_field
  36: <rustc_lint::types::VariantSizeDifferences as rustc::lint::LintPass>::name
  37: <rustc_lint::types::VariantSizeDifferences as rustc::lint::LintPass>::name
  38: <rustc_lint::types::VariantSizeDifferences as rustc::lint::LintPass>::name
  39: <rustc_lint::types::VariantSizeDifferences as rustc::lint::LintPass>::name
  40: <rustc_lint::types::VariantSizeDifferences as rustc::lint::LintPass>::name
  41: <rustc_lint::BuiltinCombinedModuleLateLintPass as rustc::lint::LateLintPass>::check_struct_field
  42: <rustc_lint::types::VariantSizeDifferences as rustc::lint::LintPass>::name
  43: <rustc_lint::BuiltinCombinedModuleLateLintPass as rustc::lint::LateLintPass>::check_struct_field
  44: <rustc_lint::types::VariantSizeDifferences as rustc::lint::LintPass>::name
  45: <rustc_lint::BuiltinCombinedModuleLateLintPass as rustc::lint::LateLintPass>::check_struct_field
  46: <rustc_lint::BuiltinCombinedModuleLateLintPass as rustc::lint::LateLintPass>::check_struct_field
  47: rustc_lint::provide
  48: <rustc_interface::util::ReplaceBodyWithLoop as syntax::mut_visit::MutVisitor>::visit_mac
  49: rustc_interface::passes::BoxedGlobalCtxt::complete
  50: <rustc_interface::util::ReplaceBodyWithLoop as syntax::mut_visit::MutVisitor>::visit_mac
  51: rustc_interface::passes::BoxedGlobalCtxt::complete
  52: rustc_interface::passes::BoxedGlobalCtxt::complete
  53: <rustc_interface::proc_macro_decls::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor>::visit_item
  54: <rustc_interface::proc_macro_decls::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor>::visit_item
  55: _rust_maybe_catch_panic
  56: <rustc_interface::util::ReplaceBodyWithLoop as syntax::mut_visit::MutVisitor>::visit_mac
  57: _rust_maybe_catch_panic
  58: rustc_interface::passes::BoxedResolver::to_resolver_outputs
  59: rustc_interface::passes::BoxedResolver::to_resolver_outputs
  60: <syntax_pos::symbol::SymbolStr as core::fmt::Display>::fmt
  61: <syntax_pos::symbol::SymbolStr as core::fmt::Display>::fmt
  62: <syntax_pos::symbol::SymbolStr as core::fmt::Display>::fmt
  63: <syntax_pos::symbol::SymbolStr as core::fmt::Display>::fmt
  64: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::compile
  65: <syntax_pos::symbol::SymbolStr as core::fmt::Display>::fmt
  66: <syntax_pos::symbol::SymbolStr as core::fmt::Display>::fmt
  67: <syntax_pos::symbol::SymbolStr as core::fmt::Display>::fmt
  68: <syntax_pos::symbol::SymbolStr as core::fmt::Display>::fmt
  69: <syntax_pos::symbol::SymbolStr as core::fmt::Display>::fmt
  70: _rust_maybe_catch_panic
  71: <syntax_pos::symbol::SymbolStr as core::fmt::Display>::fmt
  72: ZN244_$LT$std..error..$LT$impl$u20$core..convert..From$LT$alloc..string..String$GT$$u20$for$u20$alloc..boxed..Box$LT$dyn$u20$std..error..Error$u2b$core..marker..Send$u2b$core..marker..Sync$GT$$GT$..from..StringError$u20$as$u20$core..fmt..Display$GT$3fmt17
  73: std::sys::windows::thread::Thread::new
  74: BaseThreadInitThunk
  75: RtlUserThreadStart
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.41.0-nightly (ded5ee001 2019-11-13) running on x86_64-pc-windows-msvc

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [normalize_ty_after_erasing_regions] normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All, def_id: None }, value: unsafe extern "system" fn(*mut rt::gen::windows::foundation::AsyncActionProgressHandler<TProgress>, <rt::gen::windows::foundation::IAsyncActionWithProgress<TProgress> as rt::RtType>::Abi, <TProgress as rt::RtType>::Abi) -> i32 }`
#1 [lint_mod] linting module `rt::gen::windows::foundation`
#2 [analysis] running analysis passes on this crate
end of query stack
