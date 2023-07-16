plain
failures:

---- [ui] src/test/ui/impl-trait/in-trait/issue-102140.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/in-trait/issue-102140.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/in-trait/issue-102140" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/in-trait/issue-102140/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: matches!(tcx.def_kind(self.def_id), DefKind :: AssocTy | DefKind ::\n    AssocConst)', compiler/rustc_middle/src/ty/sty.rs:1180:9
stack backtrace:
   0:     0x7f1345233ed5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h991b55da1e83b112
   1:     0x7f13452a3ac8 - core::fmt::write::h3c3d382678756bc2
   2:     0x7f1345225be1 - std::io::Write::write_fmt::hc19b14f8abaf5dce
   3:     0x7f1345233ce1 - std::sys_common::backtrace::print::h73b77c087d488751
   4:     0x7f1345237024 - std::panicking::default_hook::{{closure}}::h9653e6dd75fed7e6
   5:     0x7f1345236cea - std::panicking::default_hook::h3340624c2eb9f3ad
   6:     0x7f1345c7c2d4 - rustc_driver[7326d70ea4e13ad2]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f1345237794 - std::panicking::rust_panic_with_hook::hc59c70e64ca7a24d
   8:     0x7f13452374ba - std::panicking::begin_panic_handler::{{closure}}::h0332451da9847254
   9:     0x7f13452343f4 - std::sys_common::backtrace::__rust_end_short_backtrace::hc7e5242ae85d9405
  10:     0x7f13452371a2 - rust_begin_unwind
  11:     0x7f13451e7ff3 - core::panicking::panic_fmt::ha93918a6663c82a5
  12:     0x7f13451e80cd - core::panicking::panic::h4408147c1050727f
  13:     0x7f1348a91c2e - <rustc_middle[b3ae27b7ab26a79d]::ty::sty::AliasTy>::trait_ref
  14:     0x7f1348705538 - rustc_trait_selection[845b29883bdf4b74]::traits::project::normalize_to_error
  15:     0x7f1348714561 - rustc_trait_selection[845b29883bdf4b74]::traits::project::opt_normalize_projection_type
  16:     0x7f134870b8f6 - rustc_trait_selection[845b29883bdf4b74]::traits::project::project_and_unify_type
  17:     0x7f13485545d0 - <rustc_infer[3374bd96f5bab0b6]::infer::InferCtxt>::commit_if_ok::<rustc_trait_selection[845b29883bdf4b74]::traits::project::ProjectAndUnifyResult, rustc_infer[3374bd96f5bab0b6]::traits::project::MismatchedProjectionTypes, rustc_trait_selection[845b29883bdf4b74]::traits::project::poly_project_and_unify_type::{closure#0}>
  18:     0x7f134870b466 - rustc_trait_selection[845b29883bdf4b74]::traits::project::poly_project_and_unify_type
  19:     0x7f13486cf118 - <rustc_trait_selection[845b29883bdf4b74]::traits::fulfill::FulfillProcessor>::process_projection_obligation
  20:     0x7f13486d4712 - <rustc_trait_selection[845b29883bdf4b74]::traits::fulfill::FulfillProcessor as rustc_data_structures[507443cea05b7bb9]::obligation_forest::ObligationProcessor>::process_obligation
  21:     0x7f13485c752c - <rustc_data_structures[507443cea05b7bb9]::obligation_forest::ObligationForest<rustc_trait_selection[845b29883bdf4b74]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[845b29883bdf4b74]::traits::fulfill::FulfillProcessor>
  22:     0x7f13486ce8d7 - <rustc_trait_selection[845b29883bdf4b74]::traits::fulfill::FulfillmentContext as rustc_infer[3374bd96f5bab0b6]::traits::engine::TraitEngine>::select_where_possible
  23:     0x7f134619a9b5 - <rustc_hir_typeck[c4b36bdbc3c5ca0d]::fn_ctxt::FnCtxt>::check_argument_types
  24:     0x7f134616d235 - <rustc_hir_typeck[c4b36bdbc3c5ca0d]::fn_ctxt::FnCtxt>::confirm_builtin_call
  25:     0x7f134616a72f - <rustc_hir_typeck[c4b36bdbc3c5ca0d]::fn_ctxt::FnCtxt>::check_call
  26:     0x7f13461ed472 - <rustc_hir_typeck[c4b36bdbc3c5ca0d]::fn_ctxt::FnCtxt>::check_expr_kind
  27:     0x7f13461811d1 - <rustc_hir_typeck[c4b36bdbc3c5ca0d]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  28:     0x7f13461ec342 - <rustc_hir_typeck[c4b36bdbc3c5ca0d]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  29:     0x7f13461a29f6 - <rustc_hir_typeck[c4b36bdbc3c5ca0d]::fn_ctxt::FnCtxt>::check_block_with_expected
  30:     0x7f13461ed61d - <rustc_hir_typeck[c4b36bdbc3c5ca0d]::fn_ctxt::FnCtxt>::check_expr_kind
  31:     0x7f13461811d1 - <rustc_hir_typeck[c4b36bdbc3c5ca0d]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  32:     0x7f13461ec342 - <rustc_hir_typeck[c4b36bdbc3c5ca0d]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  33:     0x7f1346182a8b - <rustc_hir_typeck[c4b36bdbc3c5ca0d]::fn_ctxt::FnCtxt>::check_return_expr
  34:     0x7f13462fcf84 - rustc_hir_typeck[c4b36bdbc3c5ca0d]::check::check_fn
  35:     0x7f13462d2e8d - rustc_hir_typeck[c4b36bdbc3c5ca0d]::typeck
  36:     0x7f1347a909f9 - rustc_query_system[7fa521f5ca1ea01c]::query::plumbing::try_execute_query::<rustc_query_impl[fdc87dbd73208a35]::plumbing::QueryCtxt, rustc_query_system[7fa521f5ca1ea01c]::query::caches::VecCache<rustc_span[8688867482b25c8b]::def_id::LocalDefId, &rustc_middle[b3ae27b7ab26a79d]::ty::context::TypeckResults>>
  37:     0x7f1347b7cc86 - rustc_query_system[7fa521f5ca1ea01c]::query::plumbing::get_query::<rustc_query_impl[fdc87dbd73208a35]::queries::typeck, rustc_query_impl[fdc87dbd73208a35]::plumbing::QueryCtxt>
  38:     0x7f134774ee80 - <rustc_query_impl[fdc87dbd73208a35]::Queries as rustc_middle[b3ae27b7ab26a79d]::ty::query::QueryEngine>::typeck
  39:     0x7f1348a2efc0 - <rustc_middle[b3ae27b7ab26a79d]::ty::context::TyCtxt>::typeck_opt_const_arg
  40:     0x7f1346e1de80 - rustc_mir_build[bdf718eeeeaf3718]::thir::cx::thir_body
  41:     0x7f1347b80a7a - rustc_query_system[7fa521f5ca1ea01c]::query::plumbing::get_query::<rustc_query_impl[fdc87dbd73208a35]::queries::thir_body, rustc_query_impl[fdc87dbd73208a35]::plumbing::QueryCtxt>
  42:     0x7f134772cd46 - <rustc_query_impl[fdc87dbd73208a35]::Queries as rustc_middle[b3ae27b7ab26a79d]::ty::query::QueryEngine>::thir_body
  43:     0x7f1346d9aa75 - rustc_mir_build[bdf718eeeeaf3718]::build::mir_built
  44:     0x7f1347a312e8 - rustc_query_system[7fa521f5ca1ea01c]::query::plumbing::try_execute_query::<rustc_query_impl[fdc87dbd73208a35]::plumbing::QueryCtxt, rustc_query_system[7fa521f5ca1ea01c]::query::caches::DefaultCache<rustc_middle[b3ae27b7ab26a79d]::ty::WithOptConstParam<rustc_span[8688867482b25c8b]::def_id::LocalDefId>, &rustc_data_structures[507443cea05b7bb9]::steal::Steal<rustc_middle[b3ae27b7ab26a79d]::mir::Body>>>
  45:     0x7f1347b7ef7c - rustc_query_system[7fa521f5ca1ea01c]::query::plumbing::get_query::<rustc_query_impl[fdc87dbd73208a35]::queries::mir_built, rustc_query_impl[fdc87dbd73208a35]::plumbing::QueryCtxt>
  46:     0x7f134772f2c4 - <rustc_query_impl[fdc87dbd73208a35]::Queries as rustc_middle[b3ae27b7ab26a79d]::ty::query::QueryEngine>::mir_built
  47:     0x7f13468f93b0 - rustc_mir_transform[221009d22664115b]::check_unsafety::unsafety_check_result
  48:     0x7f13468f4d60 - <rustc_mir_transform[221009d22664115b]::check_unsafety::provide::{closure#0} as core[85c1accd9c5665a0]::ops::function::FnOnce<(rustc_middle[b3ae27b7ab26a79d]::ty::context::TyCtxt, rustc_span[8688867482b25c8b]::def_id::LocalDefId)>>::call_once
  49:     0x7f1347a92579 - rustc_query_system[7fa521f5ca1ea01c]::query::plumbing::try_execute_query::<rustc_query_impl[fdc87dbd73208a35]::plumbing::QueryCtxt, rustc_query_system[7fa521f5ca1ea01c]::query::caches::VecCache<rustc_span[8688867482b25c8b]::def_id::LocalDefId, &rustc_middle[b3ae27b7ab26a79d]::mir::query::UnsafetyCheckResult>>
  50:     0x7f1347b50886 - rustc_query_system[7fa521f5ca1ea01c]::query::plumbing::get_query::<rustc_query_impl[fdc87dbd73208a35]::queries::unsafety_check_result, rustc_query_impl[fdc87dbd73208a35]::plumbing::QueryCtxt>
  51:     0x7f1347744420 - <rustc_query_impl[fdc87dbd73208a35]::Queries as rustc_middle[b3ae27b7ab26a79d]::ty::query::QueryEngine>::unsafety_check_result
  52:     0x7f134684c8ae - rustc_mir_transform[221009d22664115b]::mir_const
  53:     0x7f1347a312e8 - rustc_query_system[7fa521f5ca1ea01c]::query::plumbing::try_execute_query::<rustc_query_impl[fdc87dbd73208a35]::plumbing::QueryCtxt, rustc_query_system[7fa521f5ca1ea01c]::query::caches::DefaultCache<rustc_middle[b3ae27b7ab26a79d]::ty::WithOptConstParam<rustc_span[8688867482b25c8b]::def_id::LocalDefId>, &rustc_data_structures[507443cea05b7bb9]::steal::Steal<rustc_middle[b3ae27b7ab26a79d]::mir::Body>>>
  54:     0x7f1347b7f09f - rustc_query_system[7fa521f5ca1ea01c]::query::plumbing::get_query::<rustc_query_impl[fdc87dbd73208a35]::queries::mir_const, rustc_query_impl[fdc87dbd73208a35]::plumbing::QueryCtxt>
  55:     0x7f134772fa34 - <rustc_query_impl[fdc87dbd73208a35]::Queries as rustc_middle[b3ae27b7ab26a79d]::ty::query::QueryEngine>::mir_const
  56:     0x7f134684d7fd - rustc_mir_transform[221009d22664115b]::mir_promoted
  57:     0x7f1347b2dba7 - rustc_query_system[7fa521f5ca1ea01c]::query::plumbing::get_query::<rustc_query_impl[fdc87dbd73208a35]::queries::mir_promoted, rustc_query_impl[fdc87dbd73208a35]::plumbing::QueryCtxt>
  58:     0x7f1347732704 - <rustc_query_impl[fdc87dbd73208a35]::Queries as rustc_middle[b3ae27b7ab26a79d]::ty::query::QueryEngine>::mir_promoted
  59:     0x7f134708f708 - rustc_borrowck[13beed4fef5fd1ad]::mir_borrowck
  60:     0x7f134705a740 - <rustc_borrowck[13beed4fef5fd1ad]::provide::{closure#0} as core[85c1accd9c5665a0]::ops::function::FnOnce<(rustc_middle[b3ae27b7ab26a79d]::ty::context::TyCtxt, rustc_span[8688867482b25c8b]::def_id::LocalDefId)>>::call_once
  61:     0x7f1347a917b9 - rustc_query_system[7fa521f5ca1ea01c]::query::plumbing::try_execute_query::<rustc_query_impl[fdc87dbd73208a35]::plumbing::QueryCtxt, rustc_query_system[7fa521f5ca1ea01c]::query::caches::VecCache<rustc_span[8688867482b25c8b]::def_id::LocalDefId, &rustc_middle[b3ae27b7ab26a79d]::mir::query::BorrowCheckResult>>
  62:     0x7f1347b2d4d2 - rustc_query_system[7fa521f5ca1ea01c]::query::plumbing::get_query::<rustc_query_impl[fdc87dbd73208a35]::queries::mir_borrowck, rustc_query_impl[fdc87dbd73208a35]::plumbing::QueryCtxt>
  63:     0x7f1347751af0 - <rustc_query_impl[fdc87dbd73208a35]::Queries as rustc_middle[b3ae27b7ab26a79d]::ty::query::QueryEngine>::mir_borrowck
  64:     0x7f1346420615 - rustc_hir_analysis[88830afbda989518]::collect::type_of::find_opaque_ty_constraints_for_rpit
  65:     0x7f134641f8fc - rustc_hir_analysis[88830afbda989518]::collect::type_of::type_of
  66:     0x7f1347a4abcc - rustc_query_system[7fa521f5ca1ea01c]::query::plumbing::try_execute_query::<rustc_query_impl[fdc87dbd73208a35]::plumbing::QueryCtxt, rustc_query_system[7fa521f5ca1ea01c]::query::caches::DefaultCache<rustc_span[8688867482b25c8b]::def_id::DefId, rustc_middle[b3ae27b7ab26a79d]::ty::Ty>>
  67:     0x7f1347b7cf21 - rustc_query_system[7fa521f5ca1ea01c]::query::plumbing::get_query::<rustc_query_impl[fdc87dbd73208a35]::queries::type_of, rustc_query_impl[fdc87dbd73208a35]::plumbing::QueryCtxt>
  68:     0x7f1347724ec5 - <rustc_query_impl[fdc87dbd73208a35]::Queries as rustc_middle[b3ae27b7ab26a79d]::ty::query::QueryEngine>::type_of
  69:     0x7f1346486646 - rustc_hir_analysis[88830afbda989518]::check::check::check_opaque
  70:     0x7f1346489fe1 - rustc_hir_analysis[88830afbda989518]::check::check::check_item_type
  71:     0x7f13464995ca - rustc_hir_analysis[88830afbda989518]::check::check::check_mod_item_types
  72:     0x7f1347a940f4 - rustc_query_system[7fa521f5ca1ea01c]::query::plumbing::try_execute_query::<rustc_query_impl[fdc87dbd73208a35]::plumbing::QueryCtxt, rustc_query_system[7fa521f5ca1ea01c]::query::caches::VecCache<rustc_span[8688867482b25c8b]::def_id::LocalDefId, ()>>
  73:     0x7f1347b4aca3 - rustc_query_system[7fa521f5ca1ea01c]::query::plumbing::get_query::<rustc_query_impl[fdc87dbd73208a35]::queries::check_mod_item_types, rustc_query_impl[fdc87dbd73208a35]::plumbing::QueryCtxt>
  74:     0x7f134774a490 - <rustc_query_impl[fdc87dbd73208a35]::Queries as rustc_middle[b3ae27b7ab26a79d]::ty::query::QueryEngine>::check_mod_item_types
  75:     0x7f13464dc20a - <rustc_middle[b3ae27b7ab26a79d]::hir::map::Map>::for_each_module::<rustc_hir_analysis[88830afbda989518]::check_crate::{closure#6}::{closure#0}>
  76:     0x7f134649b1e2 - <rustc_session[6086cc0b06971831]::session::Session>::time::<(), rustc_hir_analysis[88830afbda989518]::check_crate::{closure#6}>
  77:     0x7f134662ec61 - rustc_hir_analysis[88830afbda989518]::check_crate
  78:     0x7f1345dc9111 - rustc_interface[cf6975c2f73c07c6]::passes::analysis
  79:     0x7f1347a658a8 - rustc_query_system[7fa521f5ca1ea01c]::query::plumbing::try_execute_query::<rustc_query_impl[fdc87dbd73208a35]::plumbing::QueryCtxt, rustc_query_system[7fa521f5ca1ea01c]::query::caches::DefaultCache<(), core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>>>
  80:     0x7f1347b7d03f - rustc_query_system[7fa521f5ca1ea01c]::query::plumbing::get_query::<rustc_query_impl[fdc87dbd73208a35]::queries::analysis, rustc_query_impl[fdc87dbd73208a35]::plumbing::QueryCtxt>
  81:     0x7f1347725daa - <rustc_query_impl[fdc87dbd73208a35]::Queries as rustc_middle[b3ae27b7ab26a79d]::ty::query::QueryEngine>::analysis
  82:     0x7f1345cd74bc - <rustc_interface[cf6975c2f73c07c6]::passes::QueryContext>::enter::<rustc_driver[7326d70ea4e13ad2]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>>
  83:     0x7f1345ce18af - <rustc_interface[cf6975c2f73c07c6]::interface::Compiler>::enter::<rustc_driver[7326d70ea4e13ad2]::run_compiler::{closure#1}::{closure#2}, core[85c1accd9c5665a0]::result::Result<core[85c1accd9c5665a0]::option::Option<rustc_interface[cf6975c2f73c07c6]::queries::Linker>, rustc_errors[4f821bf48e28a]::ErrorGuaranteed>>
  84:     0x7f1345c7da36 - rustc_span[8688867482b25c8b]::with_source_map::<core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>, rustc_interface[cf6975c2f73c07c6]::interface::run_compiler<core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>, rustc_driver[7326d70ea4e13ad2]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  85:     0x7f1345ce263a - <scoped_tls[2591675148d40811]::ScopedKey<rustc_span[8688867482b25c8b]::SessionGlobals>>::set::<rustc_interface[cf6975c2f73c07c6]::interface::run_compiler<core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>, rustc_driver[7326d70ea4e13ad2]::run_compiler::{closure#1}>::{closure#0}, core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>>
  86:     0x7f1345c9bedf - std[d83f25497a2ee551]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[cf6975c2f73c07c6]::util::run_in_thread_pool_with_globals<rustc_interface[cf6975c2f73c07c6]::interface::run_compiler<core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>, rustc_driver[7326d70ea4e13ad2]::run_compiler::{closure#1}>::{closure#0}, core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>>
  87:     0x7f1345cfcec6 - std[d83f25497a2ee551]::panicking::try::<core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>, core[85c1accd9c5665a0]::panic::unwind_safe::AssertUnwindSafe<<std[d83f25497a2ee551]::thread::Builder>::spawn_unchecked_<rustc_interface[cf6975c2f73c07c6]::util::run_in_thread_pool_with_globals<rustc_interface[cf6975c2f73c07c6]::interface::run_compiler<core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>, rustc_driver[7326d70ea4e13ad2]::run_compiler::{closure#1}>::{closure#0}, core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  88:     0x7f1345c8c7e5 - <<std[d83f25497a2ee551]::thread::Builder>::spawn_unchecked_<rustc_interface[cf6975c2f73c07c6]::util::run_in_thread_pool_with_globals<rustc_interface[cf6975c2f73c07c6]::interface::run_compiler<core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>, rustc_driver[7326d70ea4e13ad2]::run_compiler::{closure#1}>::{closure#0}, core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>>::{closure#1} as core[85c1accd9c5665a0]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  89:     0x7f1345243dae - std::sys::unix::thread::Thread::new::thread_start::h67a07a0a70ae98ed
  90:     0x7f1344fd9b43 - <unknown>
  91:     0x7f134506ba00 - <unknown>
  92:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (ddb1db3be 2022-11-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `<impl at /checkout/src/test/ui/impl-trait/in-trait/issue-102140.rs:21:1: 21:17>::other`
#1 [thir_body] building THIR for `<impl at /checkout/src/test/ui/impl-trait/in-trait/issue-102140.rs:21:1: 21:17>::other`
#2 [mir_built] building MIR for `<impl at /checkout/src/test/ui/impl-trait/in-trait/issue-102140.rs:21:1: 21:17>::other`
#3 [unsafety_check_result] unsafety-checking `<impl at /checkout/src/test/ui/impl-trait/in-trait/issue-102140.rs:21:1: 21:17>::other`
#4 [mir_const] preparing `<impl at /checkout/src/test/ui/impl-trait/in-trait/issue-102140.rs:21:1: 21:17>::other` for borrow checking
#5 [mir_promoted] processing MIR for `<impl at /checkout/src/test/ui/impl-trait/in-trait/issue-102140.rs:21:1: 21:17>::other`
#6 [mir_borrowck] borrow-checking `<impl at /checkout/src/test/ui/impl-trait/in-trait/issue-102140.rs:21:1: 21:17>::other`
#7 [type_of] computing type of `<impl at /checkout/src/test/ui/impl-trait/in-trait/issue-102140.rs:21:1: 21:17>::other::{opaque#0}`
#8 [check_mod_item_types] checking item types in top-level module
#9 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: VecMap([(OpaqueTypeKey { def_id: DefId(0:19 ~ issue_102140[70c9]::{impl#2}::other::{opaque#0}), substs: [] }, OpaqueTypeDecl { hidden_type: OpaqueHiddenType { span: /checkout/src/test/ui/impl-trait/in-trait/issue-102140.rs:22:24: 22:35 (#6), ty: _ }, origin: FnReturn(DefId(0:12 ~ issue_102140[70c9]::{impl#2}::other)) })])
   = note: delayed at compiler/rustc_infer/src/infer/opaque_types/table.rs:50:26

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1609:13
stack backtrace:
stack backtrace:
   0:     0x7f1345233ed5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h991b55da1e83b112
   1:     0x7f13452a3ac8 - core::fmt::write::h3c3d382678756bc2
   2:     0x7f1345225be1 - std::io::Write::write_fmt::hc19b14f8abaf5dce
   3:     0x7f1345233ce1 - std::sys_common::backtrace::print::h73b77c087d488751
   4:     0x7f1345237024 - std::panicking::default_hook::{{closure}}::h9653e6dd75fed7e6
   5:     0x7f1345236cea - std::panicking::default_hook::h3340624c2eb9f3ad
   6:     0x7f1345c7c2d4 - rustc_driver[7326d70ea4e13ad2]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f1345237794 - std::panicking::rust_panic_with_hook::hc59c70e64ca7a24d
   8:     0x7f1348c75003 - std[d83f25497a2ee551]::panicking::begin_panic::<rustc_errors[4f821bf48e28a]::ExplicitBug>::{closure#0}
   9:     0x7f1348c70df6 - std[d83f25497a2ee551]::sys_common::backtrace::__rust_end_short_backtrace::<std[d83f25497a2ee551]::panicking::begin_panic<rustc_errors[4f821bf48e28a]::ExplicitBug>::{closure#0}, !>
  10:     0x7f1345c42c76 - std[d83f25497a2ee551]::panicking::begin_panic::<rustc_errors[4f821bf48e28a]::ExplicitBug>
  11:     0x7f1348cd66f6 - std[d83f25497a2ee551]::panic::panic_any::<rustc_errors[4f821bf48e28a]::ExplicitBug>
  12:     0x7f1348cde9d6 - <rustc_errors[4f821bf48e28a]::HandlerInner>::flush_delayed::<alloc[90b592be2ff58d64]::vec::Vec<rustc_errors[4f821bf48e28a]::diagnostic::Diagnostic>, &str>
  13:     0x7f1348cda088 - <rustc_errors[4f821bf48e28a]::HandlerInner as core[85c1accd9c5665a0]::ops::drop::Drop>::drop
  14:     0x7f1345c96eeb - core[85c1accd9c5665a0]::ptr::drop_in_place::<rustc_session[6086cc0b06971831]::parse::ParseSess>
  15:     0x7f1345c98340 - core[85c1accd9c5665a0]::ptr::drop_in_place::<rustc_session[6086cc0b06971831]::session::Session>
  16:     0x7f1345ca0873 - <alloc[90b592be2ff58d64]::rc::Rc<rustc_session[6086cc0b06971831]::session::Session> as core[85c1accd9c5665a0]::ops::drop::Drop>::drop
  17:     0x7f1345c82443 - core[85c1accd9c5665a0]::ptr::drop_in_place::<rustc_interface[cf6975c2f73c07c6]::interface::Compiler>
  18:     0x7f1345c7ddfa - rustc_span[8688867482b25c8b]::with_source_map::<core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>, rustc_interface[cf6975c2f73c07c6]::interface::run_compiler<core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>, rustc_driver[7326d70ea4e13ad2]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  19:     0x7f1345ce263a - <scoped_tls[2591675148d40811]::ScopedKey<rustc_span[8688867482b25c8b]::SessionGlobals>>::set::<rustc_interface[cf6975c2f73c07c6]::interface::run_compiler<core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>, rustc_driver[7326d70ea4e13ad2]::run_compiler::{closure#1}>::{closure#0}, core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>>
  20:     0x7f1345c9bedf - std[d83f25497a2ee551]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[cf6975c2f73c07c6]::util::run_in_thread_pool_with_globals<rustc_interface[cf6975c2f73c07c6]::interface::run_compiler<core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>, rustc_driver[7326d70ea4e13ad2]::run_compiler::{closure#1}>::{closure#0}, core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>>
  21:     0x7f1345cfcec6 - std[d83f25497a2ee551]::panicking::try::<core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>, core[85c1accd9c5665a0]::panic::unwind_safe::AssertUnwindSafe<<std[d83f25497a2ee551]::thread::Builder>::spawn_unchecked_<rustc_interface[cf6975c2f73c07c6]::util::run_in_thread_pool_with_globals<rustc_interface[cf6975c2f73c07c6]::interface::run_compiler<core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>, rustc_driver[7326d70ea4e13ad2]::run_compiler::{closure#1}>::{closure#0}, core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  22:     0x7f1345c8c7e5 - <<std[d83f25497a2ee551]::thread::Builder>::spawn_unchecked_<rustc_interface[cf6975c2f73c07c6]::util::run_in_thread_pool_with_globals<rustc_interface[cf6975c2f73c07c6]::interface::run_compiler<core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>, rustc_driver[7326d70ea4e13ad2]::run_compiler::{closure#1}>::{closure#0}, core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[85c1accd9c5665a0]::result::Result<(), rustc_errors[4f821bf48e28a]::ErrorGuaranteed>>::{closure#1} as core[85c1accd9c5665a0]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  23:     0x7f1345243dae - std::sys::unix::thread::Thread::new::thread_start::h67a07a0a70ae98ed
  24:     0x7f1344fd9b43 - <unknown>
  25:     0x7f134506ba00 - <unknown>
  26:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.67.0-nightly (ddb1db3be 2022-11-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.
------------------------------------------
