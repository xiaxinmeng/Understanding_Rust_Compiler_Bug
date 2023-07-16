
error[E0403]: the name `A` is already used for a generic parameter in this item's generic parameters
 --> yo.rs:1:14
  |
1 | trait Foo<A, A = A> {}
  |           -  ^ already used
  |           |
  |           first use of `A`

error: internal compiler error: compiler/rustc_middle/src/ty/subst.rs:726:9: type parameter `A/#2` (A/2) out of range when substituting, substs=[FreshTy(0), T]

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/e75aab045fc476f176a58c408f6b06f0e275c6e1/compiler/rustc_errors/src/lib.rs:1551:9
stack backtrace:
   0:        0x10b5bff12 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1b594e9bcc0c6898
   1:        0x10b61e4ba - core::fmt::write::h7c6f83d024852aa9
   2:        0x10b5b1fec - std::io::Write::write_fmt::h97d969d4aea1606d
   3:        0x10b5bfcda - std::sys_common::backtrace::print::h562dadf6028256bf
   4:        0x10b5c30b6 - std::panicking::default_hook::{{closure}}::hee367e24075678e4
   5:        0x10b5c2e07 - std::panicking::default_hook::hcce3553a0befadd1
   6:        0x11cbeef3d - rustc_driver[f93b8a3886cc8b65]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:        0x10b5c38b5 - std::panicking::rust_panic_with_hook::h4f6feaafc55c56a2
   8:        0x12130c4d7 - std[38dd6138ba148cb2]::panicking::begin_panic::<rustc_errors[ef89fc2f19a46f90]::ExplicitBug>::{closure#0}
   9:        0x12130bbe9 - std[38dd6138ba148cb2]::sys_common::backtrace::__rust_end_short_backtrace::<std[38dd6138ba148cb2]::panicking::begin_panic<rustc_errors[ef89fc2f19a46f90]::ExplicitBug>::{closure#0}, !>
  10:        0x1219b7a39 - std[38dd6138ba148cb2]::panicking::begin_panic::<rustc_errors[ef89fc2f19a46f90]::ExplicitBug>
  11:        0x1213062c9 - std[38dd6138ba148cb2]::panic::panic_any::<rustc_errors[ef89fc2f19a46f90]::ExplicitBug>
  12:        0x12130453d - <rustc_errors[ef89fc2f19a46f90]::HandlerInner>::bug::<&alloc[dd1f1a43ef059f04]::string::String>
  13:        0x121304007 - <rustc_errors[ef89fc2f19a46f90]::Handler>::bug::<&alloc[dd1f1a43ef059f04]::string::String>
  14:        0x121460872 - rustc_middle[85091bb6846d2e9a]::ty::context::tls::with_context_opt::<rustc_middle[85091bb6846d2e9a]::ty::context::tls::with_opt<rustc_middle[85091bb6846d2e9a]::util::bug::opt_span_bug_fmt<rustc_span[50ad8565b2f1bbfb]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  15:        0x1214617d1 - rustc_middle[85091bb6846d2e9a]::util::bug::opt_span_bug_fmt::<rustc_span[50ad8565b2f1bbfb]::span_encoding::Span>
  16:        0x1219b93ee - rustc_middle[85091bb6846d2e9a]::util::bug::bug_fmt
  17:        0x1219b9198 - <rustc_middle[85091bb6846d2e9a]::ty::subst::SubstFolder>::type_param_out_of_range
  18:        0x121440709 - <rustc_middle[85091bb6846d2e9a]::ty::subst::SubstFolder as rustc_middle[85091bb6846d2e9a]::ty::fold::TypeFolder>::fold_ty
  19:        0x11f73fdc5 - <<dyn rustc_hir_analysis[e85e3b9a54dc24a9]::astconv::AstConv>::create_substs_for_ast_path::{closure#0}::SubstsForAstPathCtxt as rustc_hir_analysis[e85e3b9a54dc24a9]::astconv::CreateSubstsForGenericArgsCtxt>::inferred_kind
  20:        0x11f73eacf - <dyn rustc_hir_analysis[e85e3b9a54dc24a9]::astconv::AstConv>::create_substs_for_ast_path::{closure#0}
  21:        0x11f729bcc - <dyn rustc_hir_analysis[e85e3b9a54dc24a9]::astconv::AstConv>::instantiate_poly_trait_ref_inner
  22:        0x11f740e2e - <dyn rustc_hir_analysis[e85e3b9a54dc24a9]::astconv::AstConv>::instantiate_poly_trait_ref
  23:        0x11f72c228 - <dyn rustc_hir_analysis[e85e3b9a54dc24a9]::astconv::AstConv>::conv_object_ty_poly_trait_ref
  24:        0x11f74eb4f - <dyn rustc_hir_analysis[e85e3b9a54dc24a9]::astconv::AstConv>::ast_ty_to_ty_inner::{closure#0}
  25:        0x11f6775df - rustc_hir_analysis[e85e3b9a54dc24a9]::collect::type_of::type_of
  26:        0x120403c95 - rustc_query_system[38b064a260926677]::query::plumbing::try_execute_query::<rustc_query_impl[fcf86a589ba14889]::plumbing::QueryCtxt, rustc_query_system[38b064a260926677]::query::caches::DefaultCache<rustc_span[50ad8565b2f1bbfb]::def_id::DefId, rustc_middle[85091bb6846d2e9a]::ty::Ty>>
  27:        0x1204e7e24 - rustc_query_system[38b064a260926677]::query::plumbing::get_query::<rustc_query_impl[fcf86a589ba14889]::queries::type_of, rustc_query_impl[fcf86a589ba14889]::plumbing::QueryCtxt>
  28:        0x11f6b377d - rustc_hir_analysis[e85e3b9a54dc24a9]::collect::convert_item
  29:        0x11f6ac6dc - <rustc_hir_analysis[e85e3b9a54dc24a9]::collect::CollectItemTypesVisitor as rustc_hir[99b15258362c6b97]::intravisit::Visitor>::visit_item
  30:        0x11f659129 - <rustc_middle[85091bb6846d2e9a]::hir::map::Map>::visit_item_likes_in_module::<rustc_hir_analysis[e85e3b9a54dc24a9]::collect::CollectItemTypesVisitor>
  31:        0x11f6abcf5 - rustc_hir_analysis[e85e3b9a54dc24a9]::collect::collect_mod_item_types
  32:        0x1203e69f5 - rustc_query_system[38b064a260926677]::query::plumbing::try_execute_query::<rustc_query_impl[fcf86a589ba14889]::plumbing::QueryCtxt, rustc_query_system[38b064a260926677]::query::caches::DefaultCache<rustc_span[50ad8565b2f1bbfb]::def_id::LocalDefId, ()>>
  33:        0x1204dfd05 - rustc_query_system[38b064a260926677]::query::plumbing::get_query::<rustc_query_impl[fcf86a589ba14889]::queries::collect_mod_item_types, rustc_query_impl[fcf86a589ba14889]::plumbing::QueryCtxt>
  34:        0x11f6588b8 - <rustc_middle[85091bb6846d2e9a]::hir::map::Map>::for_each_module::<rustc_hir_analysis[e85e3b9a54dc24a9]::check_crate::{closure#0}::{closure#0}::{closure#0}>
  35:        0x11f648fb1 - <rustc_session[304ed9bc7c986c94]::session::Session>::track_errors::<rustc_hir_analysis[e85e3b9a54dc24a9]::check_crate::{closure#0}, ()>
  36:        0x11f767428 - rustc_hir_analysis[e85e3b9a54dc24a9]::check_crate
  37:        0x11ccb23ba - rustc_interface[bab52cec3fba6e57]::passes::analysis
  38:        0x12046db7c - rustc_query_system[38b064a260926677]::query::plumbing::try_execute_query::<rustc_query_impl[fcf86a589ba14889]::plumbing::QueryCtxt, rustc_query_system[38b064a260926677]::query::caches::DefaultCache<(), core[322641b34a183a]::result::Result<(), rustc_errors[ef89fc2f19a46f90]::ErrorGuaranteed>>>
  39:        0x1204e7f39 - rustc_query_system[38b064a260926677]::query::plumbing::get_query::<rustc_query_impl[fcf86a589ba14889]::queries::analysis, rustc_query_impl[fcf86a589ba14889]::plumbing::QueryCtxt>
  40:        0x11cb7e416 - <rustc_interface[bab52cec3fba6e57]::passes::QueryContext>::enter::<rustc_driver[f93b8a3886cc8b65]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[322641b34a183a]::result::Result<(), rustc_errors[ef89fc2f19a46f90]::ErrorGuaranteed>>
  41:        0x11cbc268d - rustc_span[50ad8565b2f1bbfb]::with_source_map::<core[322641b34a183a]::result::Result<(), rustc_errors[ef89fc2f19a46f90]::ErrorGuaranteed>, rustc_interface[bab52cec3fba6e57]::interface::run_compiler<core[322641b34a183a]::result::Result<(), rustc_errors[ef89fc2f19a46f90]::ErrorGuaranteed>, rustc_driver[f93b8a3886cc8b65]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  42:        0x11cbb25bc - <scoped_tls[5d80669e9829205e]::ScopedKey<rustc_span[50ad8565b2f1bbfb]::SessionGlobals>>::set::<rustc_interface[bab52cec3fba6e57]::interface::run_compiler<core[322641b34a183a]::result::Result<(), rustc_errors[ef89fc2f19a46f90]::ErrorGuaranteed>, rustc_driver[f93b8a3886cc8b65]::run_compiler::{closure#1}>::{closure#0}, core[322641b34a183a]::result::Result<(), rustc_errors[ef89fc2f19a46f90]::ErrorGuaranteed>>
  43:        0x11cb8137a - std[38dd6138ba148cb2]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[bab52cec3fba6e57]::util::run_in_thread_pool_with_globals<rustc_interface[bab52cec3fba6e57]::interface::run_compiler<core[322641b34a183a]::result::Result<(), rustc_errors[ef89fc2f19a46f90]::ErrorGuaranteed>, rustc_driver[f93b8a3886cc8b65]::run_compiler::{closure#1}>::{closure#0}, core[322641b34a183a]::result::Result<(), rustc_errors[ef89fc2f19a46f90]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[322641b34a183a]::result::Result<(), rustc_errors[ef89fc2f19a46f90]::ErrorGuaranteed>>
  44:        0x11cb67b4b - <<std[38dd6138ba148cb2]::thread::Builder>::spawn_unchecked_<rustc_interface[bab52cec3fba6e57]::util::run_in_thread_pool_with_globals<rustc_interface[bab52cec3fba6e57]::interface::run_compiler<core[322641b34a183a]::result::Result<(), rustc_errors[ef89fc2f19a46f90]::ErrorGuaranteed>, rustc_driver[f93b8a3886cc8b65]::run_compiler::{closure#1}>::{closure#0}, core[322641b34a183a]::result::Result<(), rustc_errors[ef89fc2f19a46f90]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[322641b34a183a]::result::Result<(), rustc_errors[ef89fc2f19a46f90]::ErrorGuaranteed>>::{closure#1} as core[322641b34a183a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:        0x10b5cc967 - std::sys::unix::thread::Thread::new::thread_start::hae9a83a2ac729f3b
  46:     0x7ff80d84d4e1 - __pthread_start

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (e75aab045 2022-11-09) running on x86_64-apple-darwin

query stack during panic:
#0 [type_of] expanding type alias `Bar`
#1 [collect_mod_item_types] collecting item types in top-level module
#2 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0403`.
