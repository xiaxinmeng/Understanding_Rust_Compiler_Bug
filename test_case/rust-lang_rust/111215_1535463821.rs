plain
...............................................i........................................  2376/14920
................F.......................................................................  2464/14920
........................................................................................  2552/14920
........................................................................................  2640/14920
..................................................................F......F..............  2728/14920
........................................................................................  2904/14920
........................................................................................  2992/14920
........................................................................................  3080/14920
........................................................................................  3168/14920
---
---- [ui] tests/ui/const-generics/const-arg-in-const-arg.rs#full stdout ----
diff of stderr:

9    |
10 LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }
-                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         
13 query stack during panic:
13 query stack during panic:
14 #0 [mir_borrowck] borrow-checking `test::{constant#3}`
15 #1 [mir_drops_elaborated_and_const_checked] elaborating drops for `test::{constant#3}`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-arg-in-const-arg.full/const-arg-in-const-arg.full.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/const-arg-in-const-arg.rs`

error in revision `full`: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/const-generics/const-arg-in-const-arg.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-arg-in-const-arg.full" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-arg-in-const-arg.full/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0794]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:32:23
   |
LL |     let _: [u8; faz::<'a>(&())]; //[min]~ ERROR generic parameters may not
   |
note: the late bound lifetime parameter is introduced here
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:24:14
   |
   |
LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }


error: internal compiler error: compiler/rustc_borrowck/src/universal_regions.rs:882:36: cannot convert `ReFree(DefId(0:14 ~ const_arg_in_const_arg[41ab]::test), BrNamed(DefId(0:15 ~ const_arg_in_const_arg[41ab]::test::'a), 'a))` to a region vid
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1643:9
stack backtrace:
stack backtrace:
   0:     0x7feaf70f74b4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd2439039c79758fb
   1:     0x7feaf715e8e8 - core::fmt::write::h5af5e6e6581c9a41
   2:     0x7feaf70ebb21 - std::io::Write::write_fmt::h826e16e30a4183cc
   3:     0x7feaf70f72c1 - std::sys_common::backtrace::print::heeb8e02a2fa0cee5
   4:     0x7feaf70fa44a - std::panicking::default_hook::{{closure}}::hf8af770900b592f3
   5:     0x7feaf70fa12c - std::panicking::default_hook::h26b66e1301914f4b
   6:     0x7feaf7bbd635 - rustc_driver_impl[57e3dca3aa66de5a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7feaf70fab57 - std::panicking::rust_panic_with_hook::h265fe6438b3f1adc
   8:     0x7feafa858c53 - std[a15b623754f10a0c]::panicking::begin_panic::<rustc_errors[5441498182dd1b6b]::ExplicitBug>::{closure#0}
   9:     0x7feafa851bf6 - std[a15b623754f10a0c]::sys_common::backtrace::__rust_end_short_backtrace::<std[a15b623754f10a0c]::panicking::begin_panic<rustc_errors[5441498182dd1b6b]::ExplicitBug>::{closure#0}, !>
  10:     0x7feaf7b55356 - std[a15b623754f10a0c]::panicking::begin_panic::<rustc_errors[5441498182dd1b6b]::ExplicitBug>
  11:     0x7feafa80c697 - <rustc_errors[5441498182dd1b6b]::HandlerInner>::bug::<alloc[626288cf5c34e68c]::string::String>
  12:     0x7feafa80b729 - <rustc_errors[5441498182dd1b6b]::Handler>::bug::<alloc[626288cf5c34e68c]::string::String>
  13:     0x7feafa9b96e7 - rustc_middle[df8508ede65fa19b]::util::bug::opt_span_bug_fmt::<rustc_span[1dc645c9ab213e45]::span_encoding::Span>::{closure#0}
  14:     0x7feafa9b862c - rustc_middle[df8508ede65fa19b]::ty::context::tls::with_opt::<rustc_middle[df8508ede65fa19b]::util::bug::opt_span_bug_fmt<rustc_span[1dc645c9ab213e45]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  15:     0x7feafa9b85b4 - rustc_middle[df8508ede65fa19b]::ty::context::tls::with_context_opt::<rustc_middle[df8508ede65fa19b]::ty::context::tls::with_opt<rustc_middle[df8508ede65fa19b]::util::bug::opt_span_bug_fmt<rustc_span[1dc645c9ab213e45]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  16:     0x7feaf7b605f2 - rustc_middle[df8508ede65fa19b]::util::bug::bug_fmt
  17:     0x7feaf90ac618 - <rustc_borrowck[3babb6259a346087]::universal_regions::UniversalRegionIndices>::to_region_vid
  18:     0x7feaf92b1590 - <rustc_borrowck[3babb6259a346087]::type_check::constraint_conversion::ConstraintConversion>::convert
  19:     0x7feaf92b33fd - <rustc_borrowck[3babb6259a346087]::type_check::constraint_conversion::ConstraintConversion>::convert_all
  20:     0x7feaf9122ac4 - <rustc_borrowck[3babb6259a346087]::type_check::TypeChecker>::push_region_constraints
  21:     0x7feaf911e68a - <rustc_borrowck[3babb6259a346087]::type_check::TypeChecker>::ascribe_user_type
  22:     0x7feaf91093c9 - rustc_borrowck[3babb6259a346087]::type_check::type_check
  23:     0x7feaf915fc39 - rustc_borrowck[3babb6259a346087]::nll::compute_regions
  24:     0x7feaf8fc5ec5 - rustc_borrowck[3babb6259a346087]::do_mir_borrowck
  25:     0x7feaf8fb1060 - rustc_borrowck[3babb6259a346087]::mir_borrowck
  26:     0x7feaf9848316 - rustc_query_system[c4a4afad1510876d]::query::plumbing::try_execute_query::<rustc_query_impl[918304657205543]::queries::mir_borrowck, rustc_query_impl[918304657205543]::plumbing::QueryCtxt>
  27:     0x7feaf968795f - rustc_query_impl[918304657205543]::get_query::mir_borrowck
  28:     0x7feaf85d0a49 - rustc_middle[df8508ede65fa19b]::ty::query::query_get_at::<rustc_query_system[c4a4afad1510876d]::query::caches::VecCache<rustc_span[1dc645c9ab213e45]::def_id::LocalDefId, rustc_middle[df8508ede65fa19b]::query::erase::Erased<[u8; 8usize]>>>
  29:     0x7feaf8607297 - rustc_mir_transform[802f63b907ff1ee1]::mir_drops_elaborated_and_const_checked
  30:     0x7feaf9971b3e - rustc_query_system[c4a4afad1510876d]::query::plumbing::try_execute_query::<rustc_query_impl[918304657205543]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[918304657205543]::plumbing::QueryCtxt>
  31:     0x7feaf966b24f - rustc_query_impl[918304657205543]::get_query::mir_drops_elaborated_and_const_checked
  32:     0x7feaf85d0a49 - rustc_middle[df8508ede65fa19b]::ty::query::query_get_at::<rustc_query_system[c4a4afad1510876d]::query::caches::VecCache<rustc_span[1dc645c9ab213e45]::def_id::LocalDefId, rustc_middle[df8508ede65fa19b]::query::erase::Erased<[u8; 8usize]>>>
  33:     0x7feaf8606fbd - rustc_mir_transform[802f63b907ff1ee1]::mir_for_ctfe
  34:     0x7feaf9849a06 - rustc_query_system[c4a4afad1510876d]::query::plumbing::try_execute_query::<rustc_query_impl[918304657205543]::queries::mir_for_ctfe, rustc_query_impl[918304657205543]::plumbing::QueryCtxt>
  35:     0x7feaf966b90e - rustc_query_impl[918304657205543]::get_query::mir_for_ctfe
  36:     0x7feaf893c4dd - rustc_middle[df8508ede65fa19b]::ty::query::query_get_at::<rustc_query_system[c4a4afad1510876d]::query::caches::DefaultCache<rustc_span[1dc645c9ab213e45]::def_id::DefId, rustc_middle[df8508ede65fa19b]::query::erase::Erased<[u8; 8usize]>>>
  37:     0x7feaf89461ad - <rustc_const_eval[e3f7c7d784b04356]::const_eval::machine::CompileTimeInterpreter as rustc_const_eval[e3f7c7d784b04356]::interpret::machine::Machine>::load_mir
  38:     0x7feaf8838a9c - <rustc_const_eval[e3f7c7d784b04356]::interpret::eval_context::InterpCx<rustc_const_eval[e3f7c7d784b04356]::const_eval::machine::CompileTimeInterpreter>>::load_mir
  39:     0x7feaf898bae6 - rustc_const_eval[e3f7c7d784b04356]::const_eval::eval_queries::eval_to_allocation_raw_provider
  40:     0x7feaf990dc72 - rustc_query_system[c4a4afad1510876d]::query::plumbing::try_execute_query::<rustc_query_impl[918304657205543]::queries::eval_to_allocation_raw, rustc_query_impl[918304657205543]::plumbing::QueryCtxt>
  41:     0x7feaf968a1b9 - rustc_query_impl[918304657205543]::get_query::eval_to_allocation_raw
  42:     0x7feaf88a5a74 - <rustc_const_eval[e3f7c7d784b04356]::provide::{closure#0} as core[e236030fafa5960d]::ops::function::FnOnce<(rustc_middle[df8508ede65fa19b]::ty::context::TyCtxt, rustc_middle[df8508ede65fa19b]::ty::ParamEnvAnd<rustc_middle[df8508ede65fa19b]::mir::interpret::GlobalId>)>>::call_once
  43:     0x7feaf98799b0 - rustc_query_system[c4a4afad1510876d]::query::plumbing::try_execute_query::<rustc_query_impl[918304657205543]::queries::eval_to_valtree, rustc_query_impl[918304657205543]::plumbing::QueryCtxt>
  44:     0x7feaf968afd9 - rustc_query_impl[918304657205543]::get_query::eval_to_valtree
  45:     0x7feafa76be8e - rustc_middle[df8508ede65fa19b]::ty::query::query_get_at::<rustc_query_system[c4a4afad1510876d]::query::caches::DefaultCache<rustc_middle[df8508ede65fa19b]::ty::ParamEnvAnd<rustc_middle[df8508ede65fa19b]::mir::interpret::GlobalId>, rustc_middle[df8508ede65fa19b]::query::erase::Erased<[u8; 24usize]>>>
  46:     0x7feafa78768d - <rustc_middle[df8508ede65fa19b]::ty::context::TyCtxt>::const_eval_global_id_for_typeck
  47:     0x7feafa7865d0 - <rustc_middle[df8508ede65fa19b]::ty::context::TyCtxt>::const_eval_resolve_for_typeck
  48:     0x7feafa71ec02 - <rustc_infer[2e00875c1b8005f5]::infer::InferCtxt>::const_eval_resolve
  49:     0x7feafa3156d2 - rustc_trait_selection[4195ac4f27007d7e]::traits::const_evaluatable::is_const_evaluatable
  50:     0x7feafa3859c8 - <rustc_trait_selection[4195ac4f27007d7e]::traits::fulfill::FulfillProcessor as rustc_data_structures[f5127f6dd34fa5e0]::obligation_forest::ObligationProcessor>::process_obligation
  51:     0x7feafa481876 - <rustc_data_structures[f5127f6dd34fa5e0]::obligation_forest::ObligationForest<rustc_trait_selection[4195ac4f27007d7e]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[4195ac4f27007d7e]::traits::fulfill::FulfillProcessor>
  52:     0x7feafa37dd64 - <rustc_trait_selection[4195ac4f27007d7e]::traits::fulfill::FulfillmentContext as rustc_infer[2e00875c1b8005f5]::traits::engine::TraitEngine>::select_where_possible
  53:     0x7feaf80a2992 - <rustc_hir_typeck[72d87cea5dae3e6e]::fn_ctxt::FnCtxt>::resolve_vars_with_obligations
  54:     0x7feaf80fbca2 - <rustc_hir_typeck[72d87cea5dae3e6e]::fn_ctxt::FnCtxt>::demand_coerce_diag
  55:     0x7feaf808cb96 - <rustc_hir_typeck[72d87cea5dae3e6e]::fn_ctxt::FnCtxt>::demand_coerce
  56:     0x7feaf80b62b2 - <rustc_hir_typeck[72d87cea5dae3e6e]::fn_ctxt::FnCtxt>::check_decl
  57:     0x7feaf80b65ce - <rustc_hir_typeck[72d87cea5dae3e6e]::fn_ctxt::FnCtxt>::check_stmt
  58:     0x7feaf80b6cc5 - <rustc_hir_typeck[72d87cea5dae3e6e]::fn_ctxt::FnCtxt>::check_block_with_expected
  59:     0x7feaf80fd5fb - <rustc_hir_typeck[72d87cea5dae3e6e]::fn_ctxt::FnCtxt>::check_expr_kind
  60:     0x7feaf8095614 - <rustc_hir_typeck[72d87cea5dae3e6e]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  61:     0x7feaf80fc442 - <rustc_hir_typeck[72d87cea5dae3e6e]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  62:     0x7feaf80974bd - <rustc_hir_typeck[72d87cea5dae3e6e]::fn_ctxt::FnCtxt>::check_return_expr
  63:     0x7feaf827b4f2 - rustc_hir_typeck[72d87cea5dae3e6e]::check::check_fn
  64:     0x7feaf81670fc - rustc_hir_typeck[72d87cea5dae3e6e]::typeck
  65:     0x7feaf9980bf6 - rustc_query_system[c4a4afad1510876d]::query::plumbing::try_execute_query::<rustc_query_impl[918304657205543]::queries::typeck, rustc_query_impl[918304657205543]::plumbing::QueryCtxt>
  66:     0x7feaf96857ff - rustc_query_impl[918304657205543]::get_query::typeck
  67:     0x7feaf81572a9 - rustc_middle[df8508ede65fa19b]::ty::query::query_get_at::<rustc_query_system[c4a4afad1510876d]::query::caches::VecCache<rustc_span[1dc645c9ab213e45]::def_id::LocalDefId, rustc_middle[df8508ede65fa19b]::query::erase::Erased<[u8; 8usize]>>>
  68:     0x7feaf8166ab2 - rustc_hir_typeck[72d87cea5dae3e6e]::used_trait_imports
  69:     0x7feaf98c8e0d - rustc_query_system[c4a4afad1510876d]::query::plumbing::try_execute_query::<rustc_query_impl[918304657205543]::queries::used_trait_imports, rustc_query_impl[918304657205543]::plumbing::QueryCtxt>
  70:     0x7feaf968655f - rustc_query_impl[918304657205543]::get_query::used_trait_imports
  71:     0x7feaf8376169 - rustc_middle[df8508ede65fa19b]::ty::query::query_get_at::<rustc_query_system[c4a4afad1510876d]::query::caches::VecCache<rustc_span[1dc645c9ab213e45]::def_id::LocalDefId, rustc_middle[df8508ede65fa19b]::query::erase::Erased<[u8; 8usize]>>>
  72:     0x7feaf838947c - rustc_hir_analysis[54e3cd862fee74d5]::check_unused::check_crate
  73:     0x7feaf84d4fd0 - rustc_hir_analysis[54e3cd862fee74d5]::check_crate
  74:     0x7feaf7c9d4a9 - rustc_interface[e4ff4a3734cc1b8d]::passes::analysis
  75:     0x7feaf9985499 - rustc_query_system[c4a4afad1510876d]::query::plumbing::try_execute_query::<rustc_query_impl[918304657205543]::queries::analysis, rustc_query_impl[918304657205543]::plumbing::QueryCtxt>
  76:     0x7feaf966197f - rustc_query_impl[918304657205543]::get_query::analysis
  77:     0x7feaf7bd9b4d - <rustc_middle[df8508ede65fa19b]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[57e3dca3aa66de5a]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>>
  78:     0x7feaf7c08452 - <rustc_interface[e4ff4a3734cc1b8d]::interface::Compiler>::enter::<rustc_driver_impl[57e3dca3aa66de5a]::run_compiler::{closure#1}::{closure#2}, core[e236030fafa5960d]::result::Result<core[e236030fafa5960d]::option::Option<rustc_interface[e4ff4a3734cc1b8d]::queries::Linker>, rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>>
  79:     0x7feaf7bd87a0 - rustc_span[1dc645c9ab213e45]::set_source_map::<core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>, rustc_interface[e4ff4a3734cc1b8d]::interface::run_compiler<core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>, rustc_driver_impl[57e3dca3aa66de5a]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  80:     0x7feaf7bcc649 - <scoped_tls[164758729fad5990]::ScopedKey<rustc_span[1dc645c9ab213e45]::SessionGlobals>>::set::<rustc_interface[e4ff4a3734cc1b8d]::interface::run_compiler<core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>, rustc_driver_impl[57e3dca3aa66de5a]::run_compiler::{closure#1}>::{closure#0}, core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>>
  81:     0x7feaf7be2d36 - std[a15b623754f10a0c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[e4ff4a3734cc1b8d]::util::run_in_thread_pool_with_globals<rustc_interface[e4ff4a3734cc1b8d]::interface::run_compiler<core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>, rustc_driver_impl[57e3dca3aa66de5a]::run_compiler::{closure#1}>::{closure#0}, core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>>
  82:     0x7feaf7c20426 - std[a15b623754f10a0c]::panicking::try::<core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>, core[e236030fafa5960d]::panic::unwind_safe::AssertUnwindSafe<<std[a15b623754f10a0c]::thread::Builder>::spawn_unchecked_<rustc_interface[e4ff4a3734cc1b8d]::util::run_in_thread_pool_with_globals<rustc_interface[e4ff4a3734cc1b8d]::interface::run_compiler<core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>, rustc_driver_impl[57e3dca3aa66de5a]::run_compiler::{closure#1}>::{closure#0}, core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  83:     0x7feaf7bd177d - <<std[a15b623754f10a0c]::thread::Builder>::spawn_unchecked_<rustc_interface[e4ff4a3734cc1b8d]::util::run_in_thread_pool_with_globals<rustc_interface[e4ff4a3734cc1b8d]::interface::run_compiler<core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>, rustc_driver_impl[57e3dca3aa66de5a]::run_compiler::{closure#1}>::{closure#0}, core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>>::{closure#1} as core[e236030fafa5960d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  84:     0x7feaf710749e - std::sys::unix::thread::Thread::new::thread_start::h6419e4989c30d621
  85:     0x7feaf6ea1b43 - <unknown>
  86:     0x7feaf6f33a00 - <unknown>
  87:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (1decfc746 2023-05-04) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `test::{constant#3}`
#1 [mir_drops_elaborated_and_const_checked] elaborating drops for `test::{constant#3}`
#2 [mir_for_ctfe] caching mir of `test::{constant#3}` for CTFE
#3 [eval_to_allocation_raw] const-evaluating + checking `test::{constant#3}`
#4 [eval_to_valtree] evaluating type-level constant
#5 [typeck] type-checking `test`
#6 [used_trait_imports] finding used_trait_imports `test`
#7 [analysis] running analysis passes on this crate
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0794`.
------------------------------------------
---
To only update this specific test, also pass `--test-args const-generics/issues/issue-77357.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/const-generics/issues/issue-77357.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-77357" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-77357/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'Normalizing Binder(([&'a u32]; c_variadic: false)->_, []) without wrapping in a `Binder`', /checkout/compiler/rustc_trait_selection/src/traits/project.rs:437:9
stack backtrace:
   0:     0x7fbd34fcf4b4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd2439039c79758fb
   1:     0x7fbd350368e8 - core::fmt::write::h5af5e6e6581c9a41
   2:     0x7fbd34fc3b21 - std::io::Write::write_fmt::h826e16e30a4183cc
   3:     0x7fbd34fcf2c1 - std::sys_common::backtrace::print::heeb8e02a2fa0cee5
   4:     0x7fbd34fd244a - std::panicking::default_hook::{{closure}}::hf8af770900b592f3
   5:     0x7fbd34fd212c - std::panicking::default_hook::h26b66e1301914f4b
   6:     0x7fbd35a95635 - rustc_driver_impl[57e3dca3aa66de5a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fbd34fd2b57 - std::panicking::rust_panic_with_hook::h265fe6438b3f1adc
   8:     0x7fbd34fd28d7 - std::panicking::begin_panic_handler::{{closure}}::h1d0688fd92b70c69
   9:     0x7fbd34fcf996 - std::sys_common::backtrace::__rust_end_short_backtrace::ha3977b3fccf497a5
  10:     0x7fbd34fd25c7 - rust_begin_unwind
  11:     0x7fbd34f870c3 - core::panicking::panic_fmt::h588abc85cde4f62d
  12:     0x7fbd36008e10 - <rustc_trait_selection[4195ac4f27007d7e]::traits::project::AssocTypeNormalizer>::fold::<rustc_middle[df8508ede65fa19b]::ty::sty::Binder<rustc_middle[df8508ede65fa19b]::ty::sty::FnSig>>
  13:     0x7fbd3600eeb9 - rustc_trait_selection[4195ac4f27007d7e]::traits::project::normalize_with_depth::<rustc_middle[df8508ede65fa19b]::ty::sty::Binder<rustc_middle[df8508ede65fa19b]::ty::sty::FnSig>>
  14:     0x7fbd3613bfb1 - <rustc_infer[2e00875c1b8005f5]::infer::at::At as rustc_trait_selection[4195ac4f27007d7e]::traits::project::NormalizeExt>::normalize::<rustc_middle[df8508ede65fa19b]::ty::sty::Binder<rustc_middle[df8508ede65fa19b]::ty::sty::FnSig>>
  15:     0x7fbd35f7c3d5 - <rustc_hir_typeck[72d87cea5dae3e6e]::fn_ctxt::FnCtxt>::normalize::<rustc_middle[df8508ede65fa19b]::ty::sty::Binder<rustc_middle[df8508ede65fa19b]::ty::sty::FnSig>>
  16:     0x7fbd35fd1fb0 - <rustc_hir_typeck[72d87cea5dae3e6e]::fn_ctxt::FnCtxt>::supplied_sig_of_closure
  17:     0x7fbd35fd0b29 - <rustc_hir_typeck[72d87cea5dae3e6e]::fn_ctxt::FnCtxt>::sig_of_closure_no_expectation
  18:     0x7fbd35fcc376 - <rustc_hir_typeck[72d87cea5dae3e6e]::fn_ctxt::FnCtxt>::check_expr_closure
  19:     0x7fbd35fd5a6f - <rustc_hir_typeck[72d87cea5dae3e6e]::fn_ctxt::FnCtxt>::check_expr_kind
  20:     0x7fbd35f6d614 - <rustc_hir_typeck[72d87cea5dae3e6e]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  21:     0x7fbd35fd4442 - <rustc_hir_typeck[72d87cea5dae3e6e]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  22:     0x7fbd35f8e76a - <rustc_hir_typeck[72d87cea5dae3e6e]::fn_ctxt::FnCtxt>::check_stmt
  23:     0x7fbd35f8ecc5 - <rustc_hir_typeck[72d87cea5dae3e6e]::fn_ctxt::FnCtxt>::check_block_with_expected
  24:     0x7fbd35fd55fb - <rustc_hir_typeck[72d87cea5dae3e6e]::fn_ctxt::FnCtxt>::check_expr_kind
  25:     0x7fbd35f6d614 - <rustc_hir_typeck[72d87cea5dae3e6e]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  26:     0x7fbd35fd4442 - <rustc_hir_typeck[72d87cea5dae3e6e]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  27:     0x7fbd35f6d21f - <rustc_hir_typeck[72d87cea5dae3e6e]::fn_ctxt::FnCtxt>::check_expr_coercible_to_type
  28:     0x7fbd3603f444 - rustc_hir_typeck[72d87cea5dae3e6e]::typeck
  29:     0x7fbd37858bf6 - rustc_query_system[c4a4afad1510876d]::query::plumbing::try_execute_query::<rustc_query_impl[918304657205543]::queries::typeck, rustc_query_impl[918304657205543]::plumbing::QueryCtxt>
  30:     0x7fbd3755d7ff - rustc_query_impl[918304657205543]::get_query::typeck
  31:     0x7fbd36d7c2ca - rustc_mir_build[f1200672a54e45b2]::thir::cx::thir_body
  32:     0x7fbd37870710 - rustc_query_system[c4a4afad1510876d]::query::plumbing::try_execute_query::<rustc_query_impl[918304657205543]::queries::thir_body, rustc_query_impl[918304657205543]::plumbing::QueryCtxt>
  33:     0x7fbd3753fccf - rustc_query_impl[918304657205543]::get_query::thir_body
  34:     0x7fbd35f197ee - rustc_ty_utils[7155dcee05f0a7d6]::consts::thir_abstract_const
  35:     0x7fbd377b4d8c - rustc_query_system[c4a4afad1510876d]::query::plumbing::try_execute_query::<rustc_query_impl[918304657205543]::queries::thir_abstract_const, rustc_query_impl[918304657205543]::plumbing::QueryCtxt>
  36:     0x7fbd37542b9e - rustc_query_impl[918304657205543]::get_query::thir_abstract_const
  37:     0x7fbd38644257 - rustc_middle[df8508ede65fa19b]::ty::query::query_get_at::<rustc_query_system[c4a4afad1510876d]::query::caches::DefaultCache<rustc_span[1dc645c9ab213e45]::def_id::DefId, rustc_middle[df8508ede65fa19b]::query::erase::Erased<[u8; 16usize]>>>
  38:     0x7fbd3864158e - <rustc_middle[df8508ede65fa19b]::ty::context::TyCtxt>::bound_abstract_const
  39:     0x7fbd38745efe - <<rustc_middle[df8508ede65fa19b]::ty::context::TyCtxt>::expand_abstract_consts::Expander as rustc_type_ir[49aed4eef8c13b3d]::fold::TypeFolder<rustc_middle[df8508ede65fa19b]::ty::context::TyCtxt>>::fold_const
  40:     0x7fbd381ed106 - rustc_trait_selection[4195ac4f27007d7e]::traits::const_evaluatable::is_const_evaluatable
  41:     0x7fbd3825d9c8 - <rustc_trait_selection[4195ac4f27007d7e]::traits::fulfill::FulfillProcessor as rustc_data_structures[f5127f6dd34fa5e0]::obligation_forest::ObligationProcessor>::process_obligation
  42:     0x7fbd38359876 - <rustc_data_structures[f5127f6dd34fa5e0]::obligation_forest::ObligationForest<rustc_trait_selection[4195ac4f27007d7e]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[4195ac4f27007d7e]::traits::fulfill::FulfillProcessor>
  43:     0x7fbd38255d64 - <rustc_trait_selection[4195ac4f27007d7e]::traits::fulfill::FulfillmentContext as rustc_infer[2e00875c1b8005f5]::traits::engine::TraitEngine>::select_where_possible
  44:     0x7fbd381dcdc2 - <dyn rustc_infer[2e00875c1b8005f5]::traits::engine::TraitEngine as rustc_infer[2e00875c1b8005f5]::traits::engine::TraitEngineExt>::select_all_or_error
  45:     0x7fbd38354d0d - <rustc_trait_selection[4195ac4f27007d7e]::traits::engine::ObligationCtxt>::select_all_or_error
  46:     0x7fbd362941f3 - rustc_hir_analysis[54e3cd862fee74d5]::check::wfcheck::check_item_fn
  47:     0x7fbd3628b236 - rustc_hir_analysis[54e3cd862fee74d5]::check::wfcheck::check_well_formed
  48:     0x7fbd3777fc37 - rustc_query_system[c4a4afad1510876d]::query::plumbing::try_execute_query::<rustc_query_impl[918304657205543]::queries::check_well_formed, rustc_query_impl[918304657205543]::plumbing::QueryCtxt>
  49:     0x7fbd37581d67 - rustc_query_impl[918304657205543]::get_query::check_well_formed
  50:     0x7fbd36355b0d - std[a15b623754f10a0c]::panicking::try::<(), core[e236030fafa5960d]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f5127f6dd34fa5e0]::sync::par_for_each_in<&[rustc_hir[61873cb5780ca329]::hir::ImplItemId], <rustc_middle[df8508ede65fa19b]::hir::ModuleItems>::par_impl_items<rustc_hir_analysis[54e3cd862fee74d5]::check::wfcheck::check_mod_type_wf::{closure#1}>::{closure#0}>::{closure#0}::{closure#0}>>
  51:     0x7fbd362da47d - rustc_data_structures[f5127f6dd34fa5e0]::sync::par_for_each_in::<&[rustc_hir[61873cb5780ca329]::hir::ItemId], <rustc_middle[df8508ede65fa19b]::hir::ModuleItems>::par_items<rustc_hir_analysis[54e3cd862fee74d5]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  52:     0x7fbd36296aac - rustc_hir_analysis[54e3cd862fee74d5]::check::wfcheck::check_mod_type_wf
  53:     0x7fbd3777e9bf - rustc_query_system[c4a4afad1510876d]::query::plumbing::try_execute_query::<rustc_query_impl[918304657205543]::queries::check_mod_type_wf, rustc_query_impl[918304657205543]::plumbing::QueryCtxt>
  54:     0x7fbd3755c427 - rustc_query_impl[918304657205543]::get_query::check_mod_type_wf
  55:     0x7fbd36355c3d - std[a15b623754f10a0c]::panicking::try::<(), core[e236030fafa5960d]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f5127f6dd34fa5e0]::sync::par_for_each_in<&[rustc_hir[61873cb5780ca329]::hir_id::OwnerId], <rustc_middle[df8508ede65fa19b]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[54e3cd862fee74d5]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  56:     0x7fbd362da5ed - rustc_data_structures[f5127f6dd34fa5e0]::sync::par_for_each_in::<&[rustc_hir[61873cb5780ca329]::hir_id::OwnerId], <rustc_middle[df8508ede65fa19b]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[54e3cd862fee74d5]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
Build completed unsuccessfully in 0:13:29
  57:     0x7fbd363ace99 - rustc_hir_analysis[54e3cd862fee74d5]::check_crate
  58:     0x7fbd35b754a9 - rustc_interface[e4ff4a3734cc1b8d]::passes::analysis
  59:     0x7fbd3785d499 - rustc_query_system[c4a4afad1510876d]::query::plumbing::try_execute_query::<rustc_query_impl[918304657205543]::queries::analysis, rustc_query_impl[918304657205543]::plumbing::QueryCtxt>
  60:     0x7fbd3753997f - rustc_query_impl[918304657205543]::get_query::analysis
  61:     0x7fbd35ab1b4d - <rustc_middle[df8508ede65fa19b]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[57e3dca3aa66de5a]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>>
  62:     0x7fbd35ae0452 - <rustc_interface[e4ff4a3734cc1b8d]::interface::Compiler>::enter::<rustc_driver_impl[57e3dca3aa66de5a]::run_compiler::{closure#1}::{closure#2}, core[e236030fafa5960d]::result::Result<core[e236030fafa5960d]::option::Option<rustc_interface[e4ff4a3734cc1b8d]::queries::Linker>, rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>>
  63:     0x7fbd35ab07a0 - rustc_span[1dc645c9ab213e45]::set_source_map::<core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>, rustc_interface[e4ff4a3734cc1b8d]::interface::run_compiler<core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>, rustc_driver_impl[57e3dca3aa66de5a]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  64:     0x7fbd35aa4649 - <scoped_tls[164758729fad5990]::ScopedKey<rustc_span[1dc645c9ab213e45]::SessionGlobals>>::set::<rustc_interface[e4ff4a3734cc1b8d]::interface::run_compiler<core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>, rustc_driver_impl[57e3dca3aa66de5a]::run_compiler::{closure#1}>::{closure#0}, core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>>
  65:     0x7fbd35abad36 - std[a15b623754f10a0c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[e4ff4a3734cc1b8d]::util::run_in_thread_pool_with_globals<rustc_interface[e4ff4a3734cc1b8d]::interface::run_compiler<core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>, rustc_driver_impl[57e3dca3aa66de5a]::run_compiler::{closure#1}>::{closure#0}, core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>>
  66:     0x7fbd35af8426 - std[a15b623754f10a0c]::panicking::try::<core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>, core[e236030fafa5960d]::panic::unwind_safe::AssertUnwindSafe<<std[a15b623754f10a0c]::thread::Builder>::spawn_unchecked_<rustc_interface[e4ff4a3734cc1b8d]::util::run_in_thread_pool_with_globals<rustc_interface[e4ff4a3734cc1b8d]::interface::run_compiler<core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>, rustc_driver_impl[57e3dca3aa66de5a]::run_compiler::{closure#1}>::{closure#0}, core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  67:     0x7fbd35aa977d - <<std[a15b623754f10a0c]::thread::Builder>::spawn_unchecked_<rustc_interface[e4ff4a3734cc1b8d]::util::run_in_thread_pool_with_globals<rustc_interface[e4ff4a3734cc1b8d]::interface::run_compiler<core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>, rustc_driver_impl[57e3dca3aa66de5a]::run_compiler::{closure#1}>::{closure#0}, core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>>::{closure#1} as core[e236030fafa5960d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  68:     0x7fbd34fdf49e - std::sys::unix::thread::Thread::new::thread_start::h6419e4989c30d621
  69:     0x7fbd34d79b43 - <unknown>
  70:     0x7fbd34e0ba00 - <unknown>
  71:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (1decfc746 2023-05-04) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `bug::{constant#0}`
#1 [thir_body] building THIR for `bug::{constant#0}`
#2 [thir_abstract_const] building an abstract representation for `bug::{constant#0}`
#3 [check_well_formed] checking that `bug` is well-formed
#4 [check_mod_type_wf] checking that types are well-formed in top-level module
#5 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] tests/ui/const-generics/issues/issue-83993.rs stdout ----
---
To only update this specific test, also pass `--test-args const-generics/issues/issue-83993.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/const-generics/issues/issue-83993.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83993" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83993/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at '`WellFormed(&'b ())` has escaping bound vars, so it cannot be wrapped in a dummy binder.', compiler/rustc_hir_typeck/src/fn_ctxt/_impl.rs:483:13
stack backtrace:
   0:     0x7f8bd704f4b4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd2439039c79758fb
   1:     0x7f8bd70b68e8 - core::fmt::write::h5af5e6e6581c9a41
   2:     0x7f8bd7043b21 - std::io::Write::write_fmt::h826e16e30a4183cc
   3:     0x7f8bd704f2c1 - std::sys_common::backtrace::print::heeb8e02a2fa0cee5
   4:     0x7f8bd705244a - std::panicking::default_hook::{{closure}}::hf8af770900b592f3
   5:     0x7f8bd705212c - std::panicking::default_hook::h26b66e1301914f4b
   6:     0x7f8bd7b15635 - rustc_driver_impl[57e3dca3aa66de5a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f8bd7052b57 - std::panicking::rust_panic_with_hook::h265fe6438b3f1adc
   8:     0x7f8bd70528d7 - std::panicking::begin_panic_handler::{{closure}}::h1d0688fd92b70c69
   9:     0x7f8bd704f996 - std::sys_common::backtrace::__rust_end_short_backtrace::ha3977b3fccf497a5
  10:     0x7f8bd70525c7 - rust_begin_unwind
  11:     0x7f8bd70070c3 - core::panicking::panic_fmt::h588abc85cde4f62d
  12:     0x7f8bd80de615 - <rustc_middle[df8508ede65fa19b]::ty::sty::Binder<rustc_middle[df8508ede65fa19b]::ty::PredicateKind>>::dummy
  13:     0x7f8bd7ffd9b5 - <rustc_hir_typeck[72d87cea5dae3e6e]::fn_ctxt::FnCtxt>::register_wf_obligation
  14:     0x7f8bd7ffcdf0 - <rustc_hir_typeck[72d87cea5dae3e6e]::fn_ctxt::FnCtxt>::to_ty
  15:     0x7f8bd81f0b60 - <rustc_hir_typeck[72d87cea5dae3e6e]::gather_locals::GatherLocalsVisitor>::declare
  16:     0x7f8bd81f11af - <rustc_hir_typeck[72d87cea5dae3e6e]::gather_locals::GatherLocalsVisitor as rustc_hir[61873cb5780ca329]::intravisit::Visitor>::visit_local
  17:     0x7f8bd81f02d4 - <rustc_hir_typeck[72d87cea5dae3e6e]::gather_locals::GatherLocalsVisitor as rustc_hir[61873cb5780ca329]::intravisit::Visitor>::visit_block
  18:     0x7f8bd80bf42e - rustc_hir_typeck[72d87cea5dae3e6e]::typeck
  19:     0x7f8bd98d8bf6 - rustc_query_system[c4a4afad1510876d]::query::plumbing::try_execute_query::<rustc_query_impl[918304657205543]::queries::typeck, rustc_query_impl[918304657205543]::plumbing::QueryCtxt>
  20:     0x7f8bd95dd7ff - rustc_query_impl[918304657205543]::get_query::typeck
  21:     0x7f8bd8dfc2ca - rustc_mir_build[f1200672a54e45b2]::thir::cx::thir_body
  22:     0x7f8bd98f0710 - rustc_query_system[c4a4afad1510876d]::query::plumbing::try_execute_query::<rustc_query_impl[918304657205543]::queries::thir_body, rustc_query_impl[918304657205543]::plumbing::QueryCtxt>
  23:     0x7f8bd95bfccf - rustc_query_impl[918304657205543]::get_query::thir_body
  24:     0x7f8bd7f997ee - rustc_ty_utils[7155dcee05f0a7d6]::consts::thir_abstract_const
  25:     0x7f8bd9834d8c - rustc_query_system[c4a4afad1510876d]::query::plumbing::try_execute_query::<rustc_query_impl[918304657205543]::queries::thir_abstract_const, rustc_query_impl[918304657205543]::plumbing::QueryCtxt>
  26:     0x7f8bd95c2b9e - rustc_query_impl[918304657205543]::get_query::thir_abstract_const
  27:     0x7f8bda6c4257 - rustc_middle[df8508ede65fa19b]::ty::query::query_get_at::<rustc_query_system[c4a4afad1510876d]::query::caches::DefaultCache<rustc_span[1dc645c9ab213e45]::def_id::DefId, rustc_middle[df8508ede65fa19b]::query::erase::Erased<[u8; 16usize]>>>
  28:     0x7f8bda6c158e - <rustc_middle[df8508ede65fa19b]::ty::context::TyCtxt>::bound_abstract_const
  29:     0x7f8bda7c5efe - <<rustc_middle[df8508ede65fa19b]::ty::context::TyCtxt>::expand_abstract_consts::Expander as rustc_type_ir[49aed4eef8c13b3d]::fold::TypeFolder<rustc_middle[df8508ede65fa19b]::ty::context::TyCtxt>>::fold_const
  30:     0x7f8bda26d106 - rustc_trait_selection[4195ac4f27007d7e]::traits::const_evaluatable::is_const_evaluatable
  31:     0x7f8bda2dd9c8 - <rustc_trait_selection[4195ac4f27007d7e]::traits::fulfill::FulfillProcessor as rustc_data_structures[f5127f6dd34fa5e0]::obligation_forest::ObligationProcessor>::process_obligation
  32:     0x7f8bda3d9876 - <rustc_data_structures[f5127f6dd34fa5e0]::obligation_forest::ObligationForest<rustc_trait_selection[4195ac4f27007d7e]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[4195ac4f27007d7e]::traits::fulfill::FulfillProcessor>
  33:     0x7f8bda2d5d64 - <rustc_trait_selection[4195ac4f27007d7e]::traits::fulfill::FulfillmentContext as rustc_infer[2e00875c1b8005f5]::traits::engine::TraitEngine>::select_where_possible
  34:     0x7f8bda25cdc2 - <dyn rustc_infer[2e00875c1b8005f5]::traits::engine::TraitEngine as rustc_infer[2e00875c1b8005f5]::traits::engine::TraitEngineExt>::select_all_or_error
  35:     0x7f8bda3d4d0d - <rustc_trait_selection[4195ac4f27007d7e]::traits::engine::ObligationCtxt>::select_all_or_error
  36:     0x7f8bd83141f3 - rustc_hir_analysis[54e3cd862fee74d5]::check::wfcheck::check_item_fn
  37:     0x7f8bd830b236 - rustc_hir_analysis[54e3cd862fee74d5]::check::wfcheck::check_well_formed
  38:     0x7f8bd97ffc37 - rustc_query_system[c4a4afad1510876d]::query::plumbing::try_execute_query::<rustc_query_impl[918304657205543]::queries::check_well_formed, rustc_query_impl[918304657205543]::plumbing::QueryCtxt>
  39:     0x7f8bd9601d67 - rustc_query_impl[918304657205543]::get_query::check_well_formed
  40:     0x7f8bd83d5b0d - std[a15b623754f10a0c]::panicking::try::<(), core[e236030fafa5960d]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f5127f6dd34fa5e0]::sync::par_for_each_in<&[rustc_hir[61873cb5780ca329]::hir::ImplItemId], <rustc_middle[df8508ede65fa19b]::hir::ModuleItems>::par_impl_items<rustc_hir_analysis[54e3cd862fee74d5]::check::wfcheck::check_mod_type_wf::{closure#1}>::{closure#0}>::{closure#0}::{closure#0}>>
  41:     0x7f8bd835a47d - rustc_data_structures[f5127f6dd34fa5e0]::sync::par_for_each_in::<&[rustc_hir[61873cb5780ca329]::hir::ItemId], <rustc_middle[df8508ede65fa19b]::hir::ModuleItems>::par_items<rustc_hir_analysis[54e3cd862fee74d5]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  42:     0x7f8bd8316aac - rustc_hir_analysis[54e3cd862fee74d5]::check::wfcheck::check_mod_type_wf
  43:     0x7f8bd97fe9bf - rustc_query_system[c4a4afad1510876d]::query::plumbing::try_execute_query::<rustc_query_impl[918304657205543]::queries::check_mod_type_wf, rustc_query_impl[918304657205543]::plumbing::QueryCtxt>
  44:     0x7f8bd95dc427 - rustc_query_impl[918304657205543]::get_query::check_mod_type_wf
  45:     0x7f8bd83d5c3d - std[a15b623754f10a0c]::panicking::try::<(), core[e236030fafa5960d]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f5127f6dd34fa5e0]::sync::par_for_each_in<&[rustc_hir[61873cb5780ca329]::hir_id::OwnerId], <rustc_middle[df8508ede65fa19b]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[54e3cd862fee74d5]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  46:     0x7f8bd835a5ed - rustc_data_structures[f5127f6dd34fa5e0]::sync::par_for_each_in::<&[rustc_hir[61873cb5780ca329]::hir_id::OwnerId], <rustc_middle[df8508ede65fa19b]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[54e3cd862fee74d5]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  47:     0x7f8bd842ce99 - rustc_hir_analysis[54e3cd862fee74d5]::check_crate
  48:     0x7f8bd7bf54a9 - rustc_interface[e4ff4a3734cc1b8d]::passes::analysis
  49:     0x7f8bd98dd499 - rustc_query_system[c4a4afad1510876d]::query::plumbing::try_execute_query::<rustc_query_impl[918304657205543]::queries::analysis, rustc_query_impl[918304657205543]::plumbing::QueryCtxt>
  50:     0x7f8bd95b997f - rustc_query_impl[918304657205543]::get_query::analysis
  51:     0x7f8bd7b31b4d - <rustc_middle[df8508ede65fa19b]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[57e3dca3aa66de5a]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>>
  52:     0x7f8bd7b60452 - <rustc_interface[e4ff4a3734cc1b8d]::interface::Compiler>::enter::<rustc_driver_impl[57e3dca3aa66de5a]::run_compiler::{closure#1}::{closure#2}, core[e236030fafa5960d]::result::Result<core[e236030fafa5960d]::option::Option<rustc_interface[e4ff4a3734cc1b8d]::queries::Linker>, rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>>
  53:     0x7f8bd7b307a0 - rustc_span[1dc645c9ab213e45]::set_source_map::<core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>, rustc_interface[e4ff4a3734cc1b8d]::interface::run_compiler<core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>, rustc_driver_impl[57e3dca3aa66de5a]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  54:     0x7f8bd7b24649 - <scoped_tls[164758729fad5990]::ScopedKey<rustc_span[1dc645c9ab213e45]::SessionGlobals>>::set::<rustc_interface[e4ff4a3734cc1b8d]::interface::run_compiler<core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>, rustc_driver_impl[57e3dca3aa66de5a]::run_compiler::{closure#1}>::{closure#0}, core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>>
  55:     0x7f8bd7b3ad36 - std[a15b623754f10a0c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[e4ff4a3734cc1b8d]::util::run_in_thread_pool_with_globals<rustc_interface[e4ff4a3734cc1b8d]::interface::run_compiler<core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>, rustc_driver_impl[57e3dca3aa66de5a]::run_compiler::{closure#1}>::{closure#0}, core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>>
  56:     0x7f8bd7b78426 - std[a15b623754f10a0c]::panicking::try::<core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>, core[e236030fafa5960d]::panic::unwind_safe::AssertUnwindSafe<<std[a15b623754f10a0c]::thread::Builder>::spawn_unchecked_<rustc_interface[e4ff4a3734cc1b8d]::util::run_in_thread_pool_with_globals<rustc_interface[e4ff4a3734cc1b8d]::interface::run_compiler<core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>, rustc_driver_impl[57e3dca3aa66de5a]::run_compiler::{closure#1}>::{closure#0}, core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  57:     0x7f8bd7b2977d - <<std[a15b623754f10a0c]::thread::Builder>::spawn_unchecked_<rustc_interface[e4ff4a3734cc1b8d]::util::run_in_thread_pool_with_globals<rustc_interface[e4ff4a3734cc1b8d]::interface::run_compiler<core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>, rustc_driver_impl[57e3dca3aa66de5a]::run_compiler::{closure#1}>::{closure#0}, core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e236030fafa5960d]::result::Result<(), rustc_span[1dc645c9ab213e45]::ErrorGuaranteed>>::{closure#1} as core[e236030fafa5960d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  58:     0x7f8bd705f49e - std::sys::unix::thread::Thread::new::thread_start::h6419e4989c30d621
  59:     0x7f8bd6df9b43 - <unknown>
  60:     0x7f8bd6e8ba00 - <unknown>
  61:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (1decfc746 2023-05-04) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `bug::{constant#0}`
#1 [thir_body] building THIR for `bug::{constant#0}`
#2 [thir_abstract_const] building an abstract representation for `bug::{constant#0}`
#3 [check_well_formed] checking that `bug` is well-formed
#4 [check_mod_type_wf] checking that types are well-formed in top-level module
#5 [analysis] running analysis passes on this crate
------------------------------------------



