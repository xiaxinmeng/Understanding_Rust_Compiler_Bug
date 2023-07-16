
PS F:\code\projects\bugs\const_generic_62536> cargo build
   Compiling const_generic_62536 v0.1.0 (F:\code\projects\bugs\const_generic_62536)
warning: the feature `const_generics` is incomplete and may cause the compiler to crash
 --> src\main.rs:1:12
  |
1 | #![feature(const_generics, const_fn)]
  |            ^^^^^^^^^^^^^^

warning: unused variable: `stack`
  --> src\main.rs:38:9
   |
38 |     let stack: TinyStack<usize, {64}> = TinyStack::new(0);
   |         ^^^^^ help: consider prefixing with an underscore: `_stack`
   |
   = note: #[warn(unused_variables)] on by default

error: array lengths can't depend on generic parameters
  --> src\main.rs:14:46
   |
14 |         TinyStack { index: 0, stack: [value; N] }
   |                                              ^

error: internal compiler error: src\librustc\ich\impls_ty.rs:213: ty::TyKind::hash_stable() - can't hash a TyVid _#1t.

thread 'rustc' panicked at 'Box<Any>', src\librustc_errors\lib.rs:649:9
stack backtrace:
   0: std::sys_common::alloc::realloc_fallback
   1: std::panicking::take_hook
   2: std::panicking::take_hook
   3: <rustc::ty::layout::LayoutCx<rustc::ty::query::TyCtxtAt> as rustc_target::abi::LayoutOf>::layout_of
   4: std::panicking::rust_panic_with_hook
   5: <rustc_errors::lock::acquire_global_lock::Guard as core::ops::drop::Drop>::drop
   6: rustc_errors::Handler::bug
   7: rustc::util::bug::bug_fmt
   8: rustc::ty::wf::object_region_bounds
   9: rustc::ty::wf::object_region_bounds
  10: rustc::ty::wf::object_region_bounds
  11: rustc::util::bug::bug_fmt
  12: rustc::util::bug::bug_fmt
  13: rustc::hir::def::NonMacroAttrKind::descr
  14: <rustc::ty::sty::TyKind as core::fmt::Debug>::fmt
  15: rustc::hir::print::State::print_is_auto
  16: <alloc::vec::Vec<(alloc::string::String, u64)> as rustc::session::config::dep_tracking::DepTrackingHash>::hash
  17: <rustc::ty::subst::UnpackedKind as core::fmt::Debug>::fmt
  18: <rustc::ty::sty::InferConst as core::fmt::Debug>::fmt
  19: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::try_print_query_stack
  20: <rustc::traits::query::type_op::ascribe_user_type::AscribeUserType as rustc::traits::query::type_op::QueryTypeOp>::perform_query
  21: <rustc_mir::dataflow::move_paths::abs_domain::AbstractOperand as core::fmt::Debug>::fmt
  22: <rustc_mir::interpret::intern::InternMode as core::fmt::Debug>::fmt
  23: <rustc_mir::borrow_check::nll::type_check::TypeVerifier as rustc::mir::visit::Visitor>::visit_body
  24: <rustc_mir::borrow_check::nll::constraint_generation::ConstraintGeneration as rustc::mir::visit::Visitor>::visit_terminator
  25: <rustc_mir::borrow_check::nll::member_constraints::MemberConstraintSet<rustc::ty::sty::RegionVid> as core::default::Default>::default
  26: rustc_mir::borrow_check::nll::universal_regions::UniversalRegionIndices::insert_late_bound_region
  27: <rustc_mir::hair::pattern::PatternRange as core::fmt::Debug>::fmt
  28: rustc_mir::borrow_check::nll::universal_regions::UniversalRegionIndices::insert_late_bound_region
  29: <rustc_interface::util::ReplaceBodyWithLoop as syntax::mut_visit::MutVisitor>::visit_mac
  30: rustc_interface::passes::BoxedGlobalCtxt::complete
  31: rustc_interface::passes::BoxedGlobalCtxt::complete
  32: rustc_interface::passes::BoxedGlobalCtxt::complete
  33: rustc_interface::passes::BoxedGlobalCtxt::complete
  34: rustc_interface::passes::BoxedResolver::to_expansion_result
  35: <rustc_privacy::ObsoleteCheckTypeForPrivatenessVisitor as rustc::hir::intravisit::Visitor>::nested_visit_map
  36: <rustc_privacy::ObsoleteCheckTypeForPrivatenessVisitor as rustc::hir::intravisit::Visitor>::nested_visit_map
  37: <rustc_lint::BuiltinCombinedPreExpansionLintPass as rustc::lint::EarlyLintPass>::check_expr_post
  38: <rustc_lint::BuiltinCombinedPreExpansionLintPass as rustc::lint::EarlyLintPass>::check_expr_post
  39: <humantime::date::Rfc3339Timestamp as core::fmt::Debug>::fmt
  40: <rustc_lint::BuiltinCombinedPreExpansionLintPass as rustc::lint::EarlyLintPass>::check_expr_post
  41: <env_logger::filter::inner::Filter as core::fmt::Display>::fmt
  42: <env_logger::filter::inner::Filter as core::fmt::Display>::fmt
  43: <rustc_driver::Compilation as core::fmt::Debug>::fmt
  44: <rustc_driver::Compilation as core::fmt::Debug>::fmt
  45: _rust_maybe_catch_panic
  46: <rustc_lint::BuiltinCombinedPreExpansionLintPass as rustc::lint::EarlyLintPass>::check_expr_post
  47: ZN244_$LT$std..error..$LT$impl$u20$core..convert..From$LT$alloc..string..String$GT$$u20$for$u20$alloc..boxed..Box$LT$dyn$u20$std..error..Error$u2b$core..marker..Send$u2b$core..marker..Sync$GT$$GT$..from..StringError$u20$as$u20$core..fmt..Display$GT$3fmt17
  48: std::sys::windows::thread::Thread::new
  49: BaseThreadInitThunk
  50: RtlUserThreadStart
query stack during panic:
#0 [mir_borrowck] processing `main`
#1 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to 2 previous errors


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.38.0-nightly (6e310f2ab 2019-07-07) running on x86_64-pc-windows-msvc

note: compiler flags: -C debuginfo=2 -C incremental -C target-cpu=native --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `const_generic_62536`.

To learn more, run the command again with --verbose.
