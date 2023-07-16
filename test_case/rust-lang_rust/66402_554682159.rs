
error: internal compiler error: src\librustc_traits\normalize_erasing_regions.rs:42: could not fully normalize `unsafe extern "system" fn(*mut rt::gen::windows::foundation::AsyncActionProgressHandler<TProgress>, *mut rt::gen::windows::foundation::IAsyncActionWithProgress<TProgress>, <TProgress as rt::RtType>::Abi) -> i32`

thread 'rustc' panicked at 'Box<Any>', src\librustc_errors\lib.rs:937:9
stack backtrace:
   0:     0x7ff8f7b955f9 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h9615aee8d9534706
   1:     0x7ff8f7bc38cb - core::fmt::write::h4e7e9a24354d8d15
   2:     0x7ff8f7b88b24 - <std::io::IoSliceMut as core::fmt::Debug>::fmt::h65ad8595172dfff9
   3:     0x7ff8f7b99c09 - std::panicking::take_hook::h9bebd6301d94f505
   4:     0x7ff8f7b9985c - std::panicking::take_hook::h9bebd6301d94f505
   5:     0x7ff8bb43e254 - rustc_driver::report_ice::h95badae2c568257b
   6:     0x7ff8f7b9a481 - std::panicking::rust_panic_with_hook::hdd518a31c6421a10
   7:     0x7ff8bd05a310 - <rustc_errors::emitter::FileWithAnnotatedLines as core::fmt::Debug>::fmt::he1b15e3a077d4089
   8:     0x7ff8bd04a48f - rustc_errors::HandlerInner::abort_if_errors_and_should_abort::h820a0d00cc2bdcd7
   9:     0x7ff8bd048f76 - rustc_errors::Handler::bug::h3fc112204cdcea6c
  10:     0x7ff8bc7e15e1 - rustc::util::bug::bug_fmt::hffab7ca216503e0b
  11:     0x7ff8bc7e11a6 - <rustc::ty::trait_def::TraitImpls as rustc_data_structures::stable_hasher::HashStable<rustc::ich::hcx::StableHashingContext>>::hash_stable::hac9b6fc99b9af4c4
  12:     0x7ff8bc7e0e0a - <rustc::ty::trait_def::TraitImpls as rustc_data_structures::stable_hasher::HashStable<rustc::ich::hcx::StableHashingContext>>::hash_stable::hac9b6fc99b9af4c4
  13:     0x7ff8bc7e0e4a - <rustc::ty::trait_def::TraitImpls as rustc_data_structures::stable_hasher::HashStable<rustc::ich::hcx::StableHashingContext>>::hash_stable::hac9b6fc99b9af4c4
  14:     0x7ff8bc7e14ef - rustc::util::bug::bug_fmt::hffab7ca216503e0b
  15:     0x7ff8bc7e144d - rustc::util::bug::bug_fmt::hffab7ca216503e0b
  16:     0x7ff8bb77e2ee - <rustc::ty::Predicate as rustc_traits::lowering::Lower<rustc::ty::sty::Binder<rustc::traits::DomainGoal>>>::lower::heba0595c4e240c74
  17:     0x7ff8bb722a0f - <rustc::ty::Predicate as rustc_traits::lowering::Lower<rustc::ty::sty::Binder<rustc::traits::DomainGoal>>>::lower::heba0595c4e240c74
  18:     0x7ff8bc97f4a8 - rustc::ty::print::pretty::<impl core::fmt::Display for rustc::ty::subst::GenericArg>::fmt::h1503a4f0e07c036b
  19:     0x7ff8bcb83984 - rustc::middle::weak_lang_items::<impl rustc::ty::context::TyCtxt>::is_weak_lang_item::h71d5905c148544fe
  20:     0x7ff8bc827443 - rustc::dep_graph::graph::DepGraph::assert_ignored::hd93fe90ac03c0026
  21:     0x7ff8bc9c772d - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::try_print_query_stack::h0b1fe7108565fdf2
  22:     0x7ff8bc9523e8 - <rustc::traits::query::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc::ty::fold::TypeFolder>::fold_ty::h53c0f192c81ef114
  23:     0x7ff8bb5adc9a - <rustc_lint::types::TypeLimits as rustc::lint::LateLintPass>::check_expr::hacf0639e102f997a
  24:     0x7ff8bb5ad63f - <rustc_lint::types::TypeLimits as rustc::lint::LateLintPass>::check_expr::hacf0639e102f997a
  25:     0x7ff8bb5add97 - <rustc_lint::types::TypeLimits as rustc::lint::LateLintPass>::check_expr::hacf0639e102f997a
  26:     0x7ff8bb5ad63f - <rustc_lint::types::TypeLimits as rustc::lint::LateLintPass>::check_expr::hacf0639e102f997a
  27:     0x7ff8bb5ae892 - <rustc_lint::types::ImproperCTypesVisitor::check_for_opaque_ty::ProhibitOpaqueTypes as rustc::ty::fold::TypeVisitor>::visit_ty::hfaf14d6489004e86
  28:     0x7ff8bb5aea37 - <rustc_lint::types::ImproperCTypesVisitor::check_for_opaque_ty::ProhibitOpaqueTypes as rustc::ty::fold::TypeVisitor>::visit_ty::hfaf14d6489004e86
  29:     0x7ff8bb58b0a7 - <rustc_lint::BuiltinCombinedModuleLateLintPass as rustc::lint::LateLintPass>::check_struct_field::h911e61fe8aa6bab5
  30:     0x7ff8bb5c157c - <rustc_lint::types::VariantSizeDifferences as rustc::lint::LintPass>::name::hbf5eb1fdfadd1e01
  31:     0x7ff8bb590b48 - <rustc_lint::BuiltinCombinedModuleLateLintPass as rustc::lint::LateLintPass>::check_struct_field::h911e61fe8aa6bab5
  32:     0x7ff8bb5b9acd - <rustc_lint::types::VariantSizeDifferences as rustc::lint::LintPass>::name::hbf5eb1fdfadd1e01
  33:     0x7ff8bb5bfc55 - <rustc_lint::types::VariantSizeDifferences as rustc::lint::LintPass>::name::hbf5eb1fdfadd1e01
  34:     0x7ff8bb5b9b2d - <rustc_lint::types::VariantSizeDifferences as rustc::lint::LintPass>::name::hbf5eb1fdfadd1e01
  35:     0x7ff8bb5be9dd - <rustc_lint::types::VariantSizeDifferences as rustc::lint::LintPass>::name::hbf5eb1fdfadd1e01
  36:     0x7ff8bb5bd144 - <rustc_lint::types::VariantSizeDifferences as rustc::lint::LintPass>::name::hbf5eb1fdfadd1e01
  37:     0x7ff8bb58b19d - <rustc_lint::BuiltinCombinedModuleLateLintPass as rustc::lint::LateLintPass>::check_struct_field::h911e61fe8aa6bab5
  38:     0x7ff8bb5ba78a - <rustc_lint::types::VariantSizeDifferences as rustc::lint::LintPass>::name::hbf5eb1fdfadd1e01
  39:     0x7ff8bb5910f0 - <rustc_lint::BuiltinCombinedModuleLateLintPass as rustc::lint::LateLintPass>::check_struct_field::h911e61fe8aa6bab5
  40:     0x7ff8bb5c1a5b - <rustc_lint::types::VariantSizeDifferences as rustc::lint::LintPass>::name::hbf5eb1fdfadd1e01
  41:     0x7ff8bb590b48 - <rustc_lint::BuiltinCombinedModuleLateLintPass as rustc::lint::LateLintPass>::check_struct_field::h911e61fe8aa6bab5
  42:     0x7ff8bb591ce0 - <rustc_lint::BuiltinCombinedModuleLateLintPass as rustc::lint::LateLintPass>::check_struct_field::h911e61fe8aa6bab5
  43:     0x7ff8bb572ae4 - rustc_lint::provide::h8a0326545c32d21a
  44:     0x7ff8bb47edcd - <rustc_interface::util::ReplaceBodyWithLoop as syntax::mut_visit::MutVisitor>::visit_mac::hc2defbc6afd6ca69
  45:     0x7ff8bb53a9ec - rustc_interface::passes::BoxedGlobalCtxt::complete::hf1c927838f053277
  46:     0x7ff8bb481476 - <rustc_interface::util::ReplaceBodyWithLoop as syntax::mut_visit::MutVisitor>::visit_mac::hc2defbc6afd6ca69
  47:     0x7ff8bb5432a4 - rustc_interface::passes::BoxedGlobalCtxt::complete::hf1c927838f053277
  48:     0x7ff8bb53ac83 - rustc_interface::passes::BoxedGlobalCtxt::complete::hf1c927838f053277
  49:     0x7ff8bb50388b - <rustc_interface::proc_macro_decls::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor>::visit_item::h5b31f66365b35fd4
  50:     0x7ff8bb50403e - <rustc_interface::proc_macro_decls::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor>::visit_item::h5b31f66365b35fd4
  51:     0x7ff8f7babfd2 - _rust_maybe_catch_panic
  52:     0x7ff8bb470a25 - <rustc_interface::util::ReplaceBodyWithLoop as syntax::mut_visit::MutVisitor>::visit_mac::hc2defbc6afd6ca69
  53:     0x7ff8f7babfd2 - _rust_maybe_catch_panic
  54:     0x7ff8bb5115bb - rustc_interface::passes::BoxedResolver::to_resolver_outputs::hbb244c1c8caf27b0
  55:     0x7ff8bb510f14 - rustc_interface::passes::BoxedResolver::to_resolver_outputs::hbb244c1c8caf27b0
  56:     0x7ff8bb40c083 - <syntax_pos::symbol::SymbolStr as core::fmt::Display>::fmt::h9e64c96623e3423f
  57:     0x7ff8bb40f6f0 - <syntax_pos::symbol::SymbolStr as core::fmt::Display>::fmt::h9e64c96623e3423f
  58:     0x7ff8bb429b09 - <syntax_pos::symbol::SymbolStr as core::fmt::Display>::fmt::h9e64c96623e3423f
  59:     0x7ff8bb422941 - <syntax_pos::symbol::SymbolStr as core::fmt::Display>::fmt::h9e64c96623e3423f
  60:     0x7ff8bb4e4bde - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::compile::hb830e812f24c82f7
  61:     0x7ff8bb42268e - <syntax_pos::symbol::SymbolStr as core::fmt::Display>::fmt::h9e64c96623e3423f
  62:     0x7ff8bb3f3d22 - <syntax_pos::symbol::SymbolStr as core::fmt::Display>::fmt::h9e64c96623e3423f
  63:     0x7ff8bb3f1cb6 - <syntax_pos::symbol::SymbolStr as core::fmt::Display>::fmt::h9e64c96623e3423f
  64:     0x7ff8bb3fc7cf - <syntax_pos::symbol::SymbolStr as core::fmt::Display>::fmt::h9e64c96623e3423f
  65:     0x7ff8bb3fd3b4 - <syntax_pos::symbol::SymbolStr as core::fmt::Display>::fmt::h9e64c96623e3423f
  66:     0x7ff8f7babfd2 - _rust_maybe_catch_panic
  67:     0x7ff8bb3fe7e2 - <syntax_pos::symbol::SymbolStr as core::fmt::Display>::fmt::h9e64c96623e3423f
  68:     0x7ff8f7b77ab7 - ZN244_$LT$std..error..$LT$impl$u20$core..convert..From$LT$alloc..string..String$GT$$u20$for$u20$alloc..boxed..Box$LT$dyn$u20$std..error..Error$u2b$core..marker..Send$u2b$core..marker..Sync$GT$$GT$..from..StringError$u20$as$u20$core..fmt..Display$GT$3fmt17
  69:     0x7ff8f7ba9897 - std::sys::windows::thread::Thread::new::h0dc1341b7562345d
  70:     0x7ff929476fd4 - BaseThreadInitThunk
  71:     0x7ff92b25a6c1 - RtlUserThreadStart

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.41.0-nightly (ded5ee001 2019-11-13) running on x86_64-pc-windows-msvc

note: compiler flags: -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [normalize_ty_after_erasing_regions] normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All, def_id: None }, value: unsafe extern "system" fn(*mut rt::gen::windows::foundation::AsyncActionProgressHandler<TProgress>, *mut rt::gen::windows::foundation::IAsyncActionWithProgress<TProgress>, <TProgress as rt::RtType>::Abi) -> i32 }`
#1 [lint_mod] linting module `rt::gen::windows::foundation`
#2 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error

error: could not compile `winrt`.
