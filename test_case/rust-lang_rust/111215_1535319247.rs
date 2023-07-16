plain
...............................................i........................................  2376/14918
.............F..........................................................................  2464/14918
........................................................................................  2552/14918
........................................................................................  2640/14918
...................................................................F......F.............  2728/14918
........................................................................................  2904/14918
........................................................................................  2992/14918
........................................................................................  3080/14918
........................................................................................  3168/14918
---
---- [ui] tests/ui/const-generics/const-arg-in-const-arg.rs#full stdout ----
diff of stderr:

9    |
10 LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }
-                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         
+                                                                                                  
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
   0:     0x7f2114acb4b4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h598034efb3077008
   1:     0x7f2114b328c8 - core::fmt::write::hd3b7b075e6f29ffa
   1:     0x7f2114b328c8 - core::fmt::write::hd3b7b075e6f29ffa
   2:     0x7f2114abfa81 - std::io::Write::write_fmt::h41ad58a948148e4a
   3:     0x7f2114acb2c1 - std::sys_common::backtrace::print::h14ba95509e26072e
   4:     0x7f2114ace44a - std::panicking::default_hook::{{closure}}::hb7b017688cf303b2
   5:     0x7f2114ace12c - std::panicking::default_hook::hc095f4a0d3ec4815
   6:     0x7f211558f305 - rustc_driver_impl[311026437090329a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f2114aceb57 - std::panicking::rust_panic_with_hook::hdf760e84d590ea0b
   8:     0x7f211822c563 - std[bb93a438393cee13]::panicking::begin_panic::<rustc_errors[1a09eb846695810d]::ExplicitBug>::{closure#0}
   9:     0x7f21182263a6 - std[bb93a438393cee13]::sys_common::backtrace::__rust_end_short_backtrace::<std[bb93a438393cee13]::panicking::begin_panic<rustc_errors[1a09eb846695810d]::ExplicitBug>::{closure#0}, !>
  10:     0x7f2115527c96 - std[bb93a438393cee13]::panicking::begin_panic::<rustc_errors[1a09eb846695810d]::ExplicitBug>
  11:     0x7f21181e0037 - <rustc_errors[1a09eb846695810d]::HandlerInner>::bug::<alloc[41a2e75ef8564230]::string::String>
  12:     0x7f21181df0c9 - <rustc_errors[1a09eb846695810d]::Handler>::bug::<alloc[41a2e75ef8564230]::string::String>
  13:     0x7f211838cfe7 - rustc_middle[bcb1ec240aa47d18]::util::bug::opt_span_bug_fmt::<rustc_span[1e4353f243e85ae]::span_encoding::Span>::{closure#0}
  14:     0x7f211838bf2c - rustc_middle[bcb1ec240aa47d18]::ty::context::tls::with_opt::<rustc_middle[bcb1ec240aa47d18]::util::bug::opt_span_bug_fmt<rustc_span[1e4353f243e85ae]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  15:     0x7f211838beb4 - rustc_middle[bcb1ec240aa47d18]::ty::context::tls::with_context_opt::<rustc_middle[bcb1ec240aa47d18]::ty::context::tls::with_opt<rustc_middle[bcb1ec240aa47d18]::util::bug::opt_span_bug_fmt<rustc_span[1e4353f243e85ae]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  16:     0x7f2115532302 - rustc_middle[bcb1ec240aa47d18]::util::bug::bug_fmt
  17:     0x7f2116a834a8 - <rustc_borrowck[5b9078b582b96432]::universal_regions::UniversalRegionIndices>::to_region_vid
  18:     0x7f2116c86b60 - <rustc_borrowck[5b9078b582b96432]::type_check::constraint_conversion::ConstraintConversion>::convert
  19:     0x7f2116c8819b - <rustc_borrowck[5b9078b582b96432]::type_check::constraint_conversion::ConstraintConversion>::convert_all
  20:     0x7f2116b301e4 - <rustc_borrowck[5b9078b582b96432]::type_check::TypeChecker>::push_region_constraints
  21:     0x7f2116b2ccb8 - <rustc_borrowck[5b9078b582b96432]::type_check::TypeChecker>::ascribe_user_type
  22:     0x7f2116b0f1af - rustc_borrowck[5b9078b582b96432]::nll::compute_regions
  23:     0x7f211699b555 - rustc_borrowck[5b9078b582b96432]::do_mir_borrowck
  24:     0x7f21169866f0 - rustc_borrowck[5b9078b582b96432]::mir_borrowck
  25:     0x7f211721d4f6 - rustc_query_system[67bb8d70703e5e0]::query::plumbing::try_execute_query::<rustc_query_impl[28bf92f7e9cf9953]::queries::mir_borrowck, rustc_query_impl[28bf92f7e9cf9953]::plumbing::QueryCtxt>
  26:     0x7f211705cb3f - rustc_query_impl[28bf92f7e9cf9953]::get_query::mir_borrowck
  27:     0x7f2115f9fc69 - rustc_middle[bcb1ec240aa47d18]::ty::query::query_get_at::<rustc_query_system[67bb8d70703e5e0]::query::caches::VecCache<rustc_span[1e4353f243e85ae]::def_id::LocalDefId, rustc_middle[bcb1ec240aa47d18]::query::erase::Erased<[u8; 8usize]>>>
  28:     0x7f2115fd79b7 - rustc_mir_transform[6f13a44e64757485]::mir_drops_elaborated_and_const_checked
  29:     0x7f2117346d1e - rustc_query_system[67bb8d70703e5e0]::query::plumbing::try_execute_query::<rustc_query_impl[28bf92f7e9cf9953]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[28bf92f7e9cf9953]::plumbing::QueryCtxt>
  30:     0x7f211704042f - rustc_query_impl[28bf92f7e9cf9953]::get_query::mir_drops_elaborated_and_const_checked
  31:     0x7f2115f9fc69 - rustc_middle[bcb1ec240aa47d18]::ty::query::query_get_at::<rustc_query_system[67bb8d70703e5e0]::query::caches::VecCache<rustc_span[1e4353f243e85ae]::def_id::LocalDefId, rustc_middle[bcb1ec240aa47d18]::query::erase::Erased<[u8; 8usize]>>>
  32:     0x7f2115fd76dd - rustc_mir_transform[6f13a44e64757485]::mir_for_ctfe
  33:     0x7f211721ebe6 - rustc_query_system[67bb8d70703e5e0]::query::plumbing::try_execute_query::<rustc_query_impl[28bf92f7e9cf9953]::queries::mir_for_ctfe, rustc_query_impl[28bf92f7e9cf9953]::plumbing::QueryCtxt>
  34:     0x7f2117040aee - rustc_query_impl[28bf92f7e9cf9953]::get_query::mir_for_ctfe
  35:     0x7f211630e1fd - rustc_middle[bcb1ec240aa47d18]::ty::query::query_get_at::<rustc_query_system[67bb8d70703e5e0]::query::caches::DefaultCache<rustc_span[1e4353f243e85ae]::def_id::DefId, rustc_middle[bcb1ec240aa47d18]::query::erase::Erased<[u8; 8usize]>>>
  36:     0x7f2116317e8d - <rustc_const_eval[99a7dc1d880b3486]::const_eval::machine::CompileTimeInterpreter as rustc_const_eval[99a7dc1d880b3486]::interpret::machine::Machine>::load_mir
  37:     0x7f211620a93c - <rustc_const_eval[99a7dc1d880b3486]::interpret::eval_context::InterpCx<rustc_const_eval[99a7dc1d880b3486]::const_eval::machine::CompileTimeInterpreter>>::load_mir
  38:     0x7f211635d7e6 - rustc_const_eval[99a7dc1d880b3486]::const_eval::eval_queries::eval_to_allocation_raw_provider
  39:     0x7f21172e2e52 - rustc_query_system[67bb8d70703e5e0]::query::plumbing::try_execute_query::<rustc_query_impl[28bf92f7e9cf9953]::queries::eval_to_allocation_raw, rustc_query_impl[28bf92f7e9cf9953]::plumbing::QueryCtxt>
  40:     0x7f211705f399 - rustc_query_impl[28bf92f7e9cf9953]::get_query::eval_to_allocation_raw
  41:     0x7f2116277694 - <rustc_const_eval[99a7dc1d880b3486]::provide::{closure#0} as core[1754ff5e11a74914]::ops::function::FnOnce<(rustc_middle[bcb1ec240aa47d18]::ty::context::TyCtxt, rustc_middle[bcb1ec240aa47d18]::ty::ParamEnvAnd<rustc_middle[bcb1ec240aa47d18]::mir::interpret::GlobalId>)>>::call_once
  42:     0x7f211724eb90 - rustc_query_system[67bb8d70703e5e0]::query::plumbing::try_execute_query::<rustc_query_impl[28bf92f7e9cf9953]::queries::eval_to_valtree, rustc_query_impl[28bf92f7e9cf9953]::plumbing::QueryCtxt>
  43:     0x7f21170601b9 - rustc_query_impl[28bf92f7e9cf9953]::get_query::eval_to_valtree
  44:     0x7f211813f82e - rustc_middle[bcb1ec240aa47d18]::ty::query::query_get_at::<rustc_query_system[67bb8d70703e5e0]::query::caches::DefaultCache<rustc_middle[bcb1ec240aa47d18]::ty::ParamEnvAnd<rustc_middle[bcb1ec240aa47d18]::mir::interpret::GlobalId>, rustc_middle[bcb1ec240aa47d18]::query::erase::Erased<[u8; 24usize]>>>
  45:     0x7f211815b02d - <rustc_middle[bcb1ec240aa47d18]::ty::context::TyCtxt>::const_eval_global_id_for_typeck
  46:     0x7f2118159f70 - <rustc_middle[bcb1ec240aa47d18]::ty::context::TyCtxt>::const_eval_resolve_for_typeck
  47:     0x7f21180f2552 - <rustc_infer[f2a32f942b1e93c4]::infer::InferCtxt>::const_eval_resolve
  48:     0x7f2117ce9402 - rustc_trait_selection[2608253ebd3b57b]::traits::const_evaluatable::is_const_evaluatable
  49:     0x7f2117d596f8 - <rustc_trait_selection[2608253ebd3b57b]::traits::fulfill::FulfillProcessor as rustc_data_structures[d5b45647156adf7c]::obligation_forest::ObligationProcessor>::process_obligation
  50:     0x7f2117ee617c - <rustc_data_structures[d5b45647156adf7c]::obligation_forest::ObligationForest<rustc_trait_selection[2608253ebd3b57b]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[2608253ebd3b57b]::traits::fulfill::FulfillProcessor>
  51:     0x7f2117d51a94 - <rustc_trait_selection[2608253ebd3b57b]::traits::fulfill::FulfillmentContext as rustc_infer[f2a32f942b1e93c4]::traits::engine::TraitEngine>::select_where_possible
  52:     0x7f2115a747f2 - <rustc_hir_typeck[bc9466e1e0d1c1a1]::fn_ctxt::FnCtxt>::resolve_vars_with_obligations
  53:     0x7f2115acdb02 - <rustc_hir_typeck[bc9466e1e0d1c1a1]::fn_ctxt::FnCtxt>::demand_coerce_diag
  54:     0x7f2115a5e9a6 - <rustc_hir_typeck[bc9466e1e0d1c1a1]::fn_ctxt::FnCtxt>::demand_coerce
  55:     0x7f2115a88112 - <rustc_hir_typeck[bc9466e1e0d1c1a1]::fn_ctxt::FnCtxt>::check_decl
  56:     0x7f2115a8842e - <rustc_hir_typeck[bc9466e1e0d1c1a1]::fn_ctxt::FnCtxt>::check_stmt
  57:     0x7f2115a88b25 - <rustc_hir_typeck[bc9466e1e0d1c1a1]::fn_ctxt::FnCtxt>::check_block_with_expected
  58:     0x7f2115acf468 - <rustc_hir_typeck[bc9466e1e0d1c1a1]::fn_ctxt::FnCtxt>::check_expr_kind
  59:     0x7f2115a67424 - <rustc_hir_typeck[bc9466e1e0d1c1a1]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  60:     0x7f2115ace2a2 - <rustc_hir_typeck[bc9466e1e0d1c1a1]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  61:     0x7f2115a692cd - <rustc_hir_typeck[bc9466e1e0d1c1a1]::fn_ctxt::FnCtxt>::check_return_expr
  62:     0x7f2115c4d072 - rustc_hir_typeck[bc9466e1e0d1c1a1]::check::check_fn
  63:     0x7f2115b3962c - rustc_hir_typeck[bc9466e1e0d1c1a1]::typeck
  64:     0x7f2117355dd6 - rustc_query_system[67bb8d70703e5e0]::query::plumbing::try_execute_query::<rustc_query_impl[28bf92f7e9cf9953]::queries::typeck, rustc_query_impl[28bf92f7e9cf9953]::plumbing::QueryCtxt>
  65:     0x7f211705a9df - rustc_query_impl[28bf92f7e9cf9953]::get_query::typeck
  66:     0x7f2115b28f79 - rustc_middle[bcb1ec240aa47d18]::ty::query::query_get_at::<rustc_query_system[67bb8d70703e5e0]::query::caches::VecCache<rustc_span[1e4353f243e85ae]::def_id::LocalDefId, rustc_middle[bcb1ec240aa47d18]::query::erase::Erased<[u8; 8usize]>>>
  67:     0x7f2115b38fe2 - rustc_hir_typeck[bc9466e1e0d1c1a1]::used_trait_imports
  68:     0x7f211729dfed - rustc_query_system[67bb8d70703e5e0]::query::plumbing::try_execute_query::<rustc_query_impl[28bf92f7e9cf9953]::queries::used_trait_imports, rustc_query_impl[28bf92f7e9cf9953]::plumbing::QueryCtxt>
  69:     0x7f211705b73f - rustc_query_impl[28bf92f7e9cf9953]::get_query::used_trait_imports
  70:     0x7f2115d48ac9 - rustc_middle[bcb1ec240aa47d18]::ty::query::query_get_at::<rustc_query_system[67bb8d70703e5e0]::query::caches::VecCache<rustc_hir[5fdcf93fb69859c1]::hir_id::OwnerId, rustc_middle[bcb1ec240aa47d18]::query::erase::Erased<[u8; 8usize]>>>
  71:     0x7f2115d5b11c - rustc_hir_analysis[a292bf8fb7a5e941]::check_unused::check_crate
  72:     0x7f2115ea6c70 - rustc_hir_analysis[a292bf8fb7a5e941]::check_crate
  73:     0x7f211566ecc9 - rustc_interface[6bd45d958f65e896]::passes::analysis
  74:     0x7f211735a679 - rustc_query_system[67bb8d70703e5e0]::query::plumbing::try_execute_query::<rustc_query_impl[28bf92f7e9cf9953]::queries::analysis, rustc_query_impl[28bf92f7e9cf9953]::plumbing::QueryCtxt>
  75:     0x7f2117036b5f - rustc_query_impl[28bf92f7e9cf9953]::get_query::analysis
  76:     0x7f21155ac02d - <rustc_middle[bcb1ec240aa47d18]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[311026437090329a]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>>
  77:     0x7f21155da4f2 - <rustc_interface[6bd45d958f65e896]::interface::Compiler>::enter::<rustc_driver_impl[311026437090329a]::run_compiler::{closure#1}::{closure#2}, core[1754ff5e11a74914]::result::Result<core[1754ff5e11a74914]::option::Option<rustc_interface[6bd45d958f65e896]::queries::Linker>, rustc_span[1e4353f243e85ae]::ErrorGuaranteed>>
  78:     0x7f21155a50d0 - rustc_span[1e4353f243e85ae]::set_source_map::<core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>, rustc_interface[6bd45d958f65e896]::interface::run_compiler<core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>, rustc_driver_impl[311026437090329a]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  79:     0x7f211559e379 - <scoped_tls[f7fb0bc843e0be75]::ScopedKey<rustc_span[1e4353f243e85ae]::SessionGlobals>>::set::<rustc_interface[6bd45d958f65e896]::interface::run_compiler<core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>, rustc_driver_impl[311026437090329a]::run_compiler::{closure#1}>::{closure#0}, core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>>
  80:     0x7f21155b4bc6 - std[bb93a438393cee13]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6bd45d958f65e896]::util::run_in_thread_pool_with_globals<rustc_interface[6bd45d958f65e896]::interface::run_compiler<core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>, rustc_driver_impl[311026437090329a]::run_compiler::{closure#1}>::{closure#0}, core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>>
  81:     0x7f21155f4b76 - std[bb93a438393cee13]::panicking::try::<core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>, core[1754ff5e11a74914]::panic::unwind_safe::AssertUnwindSafe<<std[bb93a438393cee13]::thread::Builder>::spawn_unchecked_<rustc_interface[6bd45d958f65e896]::util::run_in_thread_pool_with_globals<rustc_interface[6bd45d958f65e896]::interface::run_compiler<core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>, rustc_driver_impl[311026437090329a]::run_compiler::{closure#1}>::{closure#0}, core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  82:     0x7f21155a34ad - <<std[bb93a438393cee13]::thread::Builder>::spawn_unchecked_<rustc_interface[6bd45d958f65e896]::util::run_in_thread_pool_with_globals<rustc_interface[6bd45d958f65e896]::interface::run_compiler<core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>, rustc_driver_impl[311026437090329a]::run_compiler::{closure#1}>::{closure#0}, core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>>::{closure#1} as core[1754ff5e11a74914]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  83:     0x7f2114adb49e - std::sys::unix::thread::Thread::new::thread_start::h5470a0151f04427f
  84:     0x7f2114875b43 - <unknown>
  85:     0x7f2114907a00 - <unknown>
  86:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (1c19446d0 2023-05-04) running on x86_64-unknown-linux-gnu


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
   0:     0x7fad60c634b4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h598034efb3077008
   1:     0x7fad60cca8c8 - core::fmt::write::hd3b7b075e6f29ffa
   2:     0x7fad60c57a81 - std::io::Write::write_fmt::h41ad58a948148e4a
   3:     0x7fad60c632c1 - std::sys_common::backtrace::print::h14ba95509e26072e
   3:     0x7fad60c632c1 - std::sys_common::backtrace::print::h14ba95509e26072e
   4:     0x7fad60c6644a - std::panicking::default_hook::{{closure}}::hb7b017688cf303b2
   5:     0x7fad60c6612c - std::panicking::default_hook::hc095f4a0d3ec4815
   6:     0x7fad61727305 - rustc_driver_impl[311026437090329a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fad60c66b57 - std::panicking::rust_panic_with_hook::hdf760e84d590ea0b
   8:     0x7fad60c668d7 - std::panicking::begin_panic_handler::{{closure}}::h93882e3b4c08a379
   9:     0x7fad60c63996 - std::sys_common::backtrace::__rust_end_short_backtrace::ha45ec159c116063f
  10:     0x7fad60c665c7 - rust_begin_unwind
  11:     0x7fad60c1b0c3 - core::panicking::panic_fmt::h751a2d43b7aaf7e4
  12:     0x7fad61c9a870 - <rustc_trait_selection[2608253ebd3b57b]::traits::project::AssocTypeNormalizer>::fold::<rustc_middle[bcb1ec240aa47d18]::ty::sty::Binder<rustc_middle[bcb1ec240aa47d18]::ty::sty::FnSig>>
  13:     0x7fad61ca0ba9 - rustc_trait_selection[2608253ebd3b57b]::traits::project::normalize_with_depth::<rustc_middle[bcb1ec240aa47d18]::ty::sty::Binder<rustc_middle[bcb1ec240aa47d18]::ty::sty::FnSig>>
  14:     0x7fad61dce211 - <rustc_infer[f2a32f942b1e93c4]::infer::at::At as rustc_trait_selection[2608253ebd3b57b]::traits::project::NormalizeExt>::normalize::<rustc_middle[bcb1ec240aa47d18]::ty::sty::Binder<rustc_middle[bcb1ec240aa47d18]::ty::sty::FnSig>>
  15:     0x7fad61c0e235 - <rustc_hir_typeck[bc9466e1e0d1c1a1]::fn_ctxt::FnCtxt>::normalize::<rustc_middle[bcb1ec240aa47d18]::ty::sty::Binder<rustc_middle[bcb1ec240aa47d18]::ty::sty::FnSig>>
  16:     0x7fad61c63e10 - <rustc_hir_typeck[bc9466e1e0d1c1a1]::fn_ctxt::FnCtxt>::supplied_sig_of_closure
  17:     0x7fad61c62989 - <rustc_hir_typeck[bc9466e1e0d1c1a1]::fn_ctxt::FnCtxt>::sig_of_closure_no_expectation
  18:     0x7fad61c5e1d6 - <rustc_hir_typeck[bc9466e1e0d1c1a1]::fn_ctxt::FnCtxt>::check_expr_closure
  19:     0x7fad61c678d5 - <rustc_hir_typeck[bc9466e1e0d1c1a1]::fn_ctxt::FnCtxt>::check_expr_kind
  20:     0x7fad61bff424 - <rustc_hir_typeck[bc9466e1e0d1c1a1]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  21:     0x7fad61c662a2 - <rustc_hir_typeck[bc9466e1e0d1c1a1]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  22:     0x7fad61c205ca - <rustc_hir_typeck[bc9466e1e0d1c1a1]::fn_ctxt::FnCtxt>::check_stmt
  23:     0x7fad61c20b25 - <rustc_hir_typeck[bc9466e1e0d1c1a1]::fn_ctxt::FnCtxt>::check_block_with_expected
  24:     0x7fad61c67468 - <rustc_hir_typeck[bc9466e1e0d1c1a1]::fn_ctxt::FnCtxt>::check_expr_kind
  25:     0x7fad61bff424 - <rustc_hir_typeck[bc9466e1e0d1c1a1]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  26:     0x7fad61c662a2 - <rustc_hir_typeck[bc9466e1e0d1c1a1]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  27:     0x7fad61bff02f - <rustc_hir_typeck[bc9466e1e0d1c1a1]::fn_ctxt::FnCtxt>::check_expr_coercible_to_type
  28:     0x7fad61cd1974 - rustc_hir_typeck[bc9466e1e0d1c1a1]::typeck
  29:     0x7fad634eddd6 - rustc_query_system[67bb8d70703e5e0]::query::plumbing::try_execute_query::<rustc_query_impl[28bf92f7e9cf9953]::queries::typeck, rustc_query_impl[28bf92f7e9cf9953]::plumbing::QueryCtxt>
  30:     0x7fad631f29df - rustc_query_impl[28bf92f7e9cf9953]::get_query::typeck
  31:     0x7fad62a11b0a - rustc_mir_build[9d9a80ededb6df87]::thir::cx::thir_body
  32:     0x7fad635058f0 - rustc_query_system[67bb8d70703e5e0]::query::plumbing::try_execute_query::<rustc_query_impl[28bf92f7e9cf9953]::queries::thir_body, rustc_query_impl[28bf92f7e9cf9953]::plumbing::QueryCtxt>
  33:     0x7fad631d4eaf - rustc_query_impl[28bf92f7e9cf9953]::get_query::thir_body
  34:     0x7fad61bab5be - rustc_ty_utils[9d90aa19451204b]::consts::thir_abstract_const
  35:     0x7fad63449f6c - rustc_query_system[67bb8d70703e5e0]::query::plumbing::try_execute_query::<rustc_query_impl[28bf92f7e9cf9953]::queries::thir_abstract_const, rustc_query_impl[28bf92f7e9cf9953]::plumbing::QueryCtxt>
  36:     0x7fad631d7d7e - rustc_query_impl[28bf92f7e9cf9953]::get_query::thir_abstract_const
  37:     0x7fad642d7bf7 - rustc_middle[bcb1ec240aa47d18]::ty::query::query_get_at::<rustc_query_system[67bb8d70703e5e0]::query::caches::DefaultCache<rustc_span[1e4353f243e85ae]::def_id::DefId, rustc_middle[bcb1ec240aa47d18]::query::erase::Erased<[u8; 16usize]>>>
  38:     0x7fad642d4f4e - <rustc_middle[bcb1ec240aa47d18]::ty::context::TyCtxt>::bound_abstract_const
  39:     0x7fad643d98be - <<rustc_middle[bcb1ec240aa47d18]::ty::context::TyCtxt>::expand_abstract_consts::Expander as rustc_type_ir[c475fa6bbe20e769]::fold::TypeFolder<rustc_middle[bcb1ec240aa47d18]::ty::context::TyCtxt>>::fold_const
  40:     0x7fad63e80e36 - rustc_trait_selection[2608253ebd3b57b]::traits::const_evaluatable::is_const_evaluatable
  41:     0x7fad63ef16f8 - <rustc_trait_selection[2608253ebd3b57b]::traits::fulfill::FulfillProcessor as rustc_data_structures[d5b45647156adf7c]::obligation_forest::ObligationProcessor>::process_obligation
  42:     0x7fad6407e17c - <rustc_data_structures[d5b45647156adf7c]::obligation_forest::ObligationForest<rustc_trait_selection[2608253ebd3b57b]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[2608253ebd3b57b]::traits::fulfill::FulfillProcessor>
  43:     0x7fad63ee9a94 - <rustc_trait_selection[2608253ebd3b57b]::traits::fulfill::FulfillmentContext as rustc_infer[f2a32f942b1e93c4]::traits::engine::TraitEngine>::select_where_possible
  44:     0x7fad63e71702 - <dyn rustc_infer[f2a32f942b1e93c4]::traits::engine::TraitEngine as rustc_infer[f2a32f942b1e93c4]::traits::engine::TraitEngineExt>::select_all_or_error
  45:     0x7fad63fe89bd - <rustc_trait_selection[2608253ebd3b57b]::traits::engine::ObligationCtxt>::select_all_or_error
  46:     0x7fad61f25e93 - rustc_hir_analysis[a292bf8fb7a5e941]::check::wfcheck::check_item_fn
  47:     0x7fad61f1ced6 - rustc_hir_analysis[a292bf8fb7a5e941]::check::wfcheck::check_well_formed
  48:     0x7fad63414e17 - rustc_query_system[67bb8d70703e5e0]::query::plumbing::try_execute_query::<rustc_query_impl[28bf92f7e9cf9953]::queries::check_well_formed, rustc_query_impl[28bf92f7e9cf9953]::plumbing::QueryCtxt>
  49:     0x7fad63216f47 - rustc_query_impl[28bf92f7e9cf9953]::get_query::check_well_formed
  50:     0x7fad61fe8acd - std[bb93a438393cee13]::panicking::try::<(), core[1754ff5e11a74914]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[d5b45647156adf7c]::sync::par_for_each_in<&[rustc_hir[5fdcf93fb69859c1]::hir::ItemId], <rustc_middle[bcb1ec240aa47d18]::hir::ModuleItems>::par_items<rustc_hir_analysis[a292bf8fb7a5e941]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  51:     0x7fad61f6c0fd - rustc_data_structures[d5b45647156adf7c]::sync::par_for_each_in::<&[rustc_hir[5fdcf93fb69859c1]::hir::ItemId], <rustc_middle[bcb1ec240aa47d18]::hir::ModuleItems>::par_items<rustc_hir_analysis[a292bf8fb7a5e941]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  52:     0x7fad61f2874c - rustc_hir_analysis[a292bf8fb7a5e941]::check::wfcheck::check_mod_type_wf
  53:     0x7fad63413b9f - rustc_query_system[67bb8d70703e5e0]::query::plumbing::try_execute_query::<rustc_query_impl[28bf92f7e9cf9953]::queries::check_mod_type_wf, rustc_query_impl[28bf92f7e9cf9953]::plumbing::QueryCtxt>
  54:     0x7fad631f1607 - rustc_query_impl[28bf92f7e9cf9953]::get_query::check_mod_type_wf
  55:     0x7fad61fe8bfd - std[bb93a438393cee13]::panicking::try::<(), core[1754ff5e11a74914]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[d5b45647156adf7c]::sync::par_for_each_in<&[rustc_hir[5fdcf93fb69859c1]::hir_id::OwnerId], <rustc_middle[bcb1ec240aa47d18]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[a292bf8fb7a5e941]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  56:     0x7fad61f6c26d - rustc_data_structures[d5b45647156adf7c]::sync::par_for_each_in::<&[rustc_hir[5fdcf93fb69859c1]::hir_id::OwnerId], <rustc_middle[bcb1ec240aa47d18]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[a292bf8fb7a5e941]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  57:     0x7fad6203eb39 - rustc_hir_analysis[a292bf8fb7a5e941]::check_crate
  58:     0x7fad61806cc9 - rustc_interface[6bd45d958f65e896]::passes::analysis
  59:     0x7fad634f2679 - rustc_query_system[67bb8d70703e5e0]::query::plumbing::try_execute_query::<rustc_query_impl[28bf92f7e9cf9953]::queries::analysis, rustc_query_impl[28bf92f7e9cf9953]::plumbing::QueryCtxt>
  60:     0x7fad631ceb5f - rustc_query_impl[28bf92f7e9cf9953]::get_query::analysis
  61:     0x7fad6174402d - <rustc_middle[bcb1ec240aa47d18]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[311026437090329a]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>>
  62:     0x7fad617724f2 - <rustc_interface[6bd45d958f65e896]::interface::Compiler>::enter::<rustc_driver_impl[311026437090329a]::run_compiler::{closure#1}::{closure#2}, core[1754ff5e11a74914]::result::Result<core[1754ff5e11a74914]::option::Option<rustc_interface[6bd45d958f65e896]::queries::Linker>, rustc_span[1e4353f243e85ae]::ErrorGuaranteed>>
  63:     0x7fad6173d0d0 - rustc_span[1e4353f243e85ae]::set_source_map::<core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>, rustc_interface[6bd45d958f65e896]::interface::run_compiler<core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>, rustc_driver_impl[311026437090329a]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  64:     0x7fad61736379 - <scoped_tls[f7fb0bc843e0be75]::ScopedKey<rustc_span[1e4353f243e85ae]::SessionGlobals>>::set::<rustc_interface[6bd45d958f65e896]::interface::run_compiler<core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>, rustc_driver_impl[311026437090329a]::run_compiler::{closure#1}>::{closure#0}, core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>>
  65:     0x7fad6174cbc6 - std[bb93a438393cee13]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6bd45d958f65e896]::util::run_in_thread_pool_with_globals<rustc_interface[6bd45d958f65e896]::interface::run_compiler<core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>, rustc_driver_impl[311026437090329a]::run_compiler::{closure#1}>::{closure#0}, core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>>
  66:     0x7fad6178cb76 - std[bb93a438393cee13]::panicking::try::<core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>, core[1754ff5e11a74914]::panic::unwind_safe::AssertUnwindSafe<<std[bb93a438393cee13]::thread::Builder>::spawn_unchecked_<rustc_interface[6bd45d958f65e896]::util::run_in_thread_pool_with_globals<rustc_interface[6bd45d958f65e896]::interface::run_compiler<core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>, rustc_driver_impl[311026437090329a]::run_compiler::{closure#1}>::{closure#0}, core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  67:     0x7fad6173b4ad - <<std[bb93a438393cee13]::thread::Builder>::spawn_unchecked_<rustc_interface[6bd45d958f65e896]::util::run_in_thread_pool_with_globals<rustc_interface[6bd45d958f65e896]::interface::run_compiler<core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>, rustc_driver_impl[311026437090329a]::run_compiler::{closure#1}>::{closure#0}, core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>>::{closure#1} as core[1754ff5e11a74914]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  68:     0x7fad60c7349e - std::sys::unix::thread::Thread::new::thread_start::h5470a0151f04427f
  69:     0x7fad60a0db43 - <unknown>
  70:     0x7fad60a9fa00 - <unknown>
  71:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (1c19446d0 2023-05-04) running on x86_64-unknown-linux-gnu

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
   0:     0x7f4bb298e4b4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h598034efb3077008
   1:     0x7f4bb29f58c8 - core::fmt::write::hd3b7b075e6f29ffa
   2:     0x7f4bb2982a81 - std::io::Write::write_fmt::h41ad58a948148e4a
   3:     0x7f4bb298e2c1 - std::sys_common::backtrace::print::h14ba95509e26072e
   3:     0x7f4bb298e2c1 - std::sys_common::backtrace::print::h14ba95509e26072e
   4:     0x7f4bb299144a - std::panicking::default_hook::{{closure}}::hb7b017688cf303b2
   5:     0x7f4bb299112c - std::panicking::default_hook::hc095f4a0d3ec4815
   6:     0x7f4bb3452305 - rustc_driver_impl[311026437090329a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f4bb2991b57 - std::panicking::rust_panic_with_hook::hdf760e84d590ea0b
   8:     0x7f4bb29918d7 - std::panicking::begin_panic_handler::{{closure}}::h93882e3b4c08a379
   9:     0x7f4bb298e996 - std::sys_common::backtrace::__rust_end_short_backtrace::ha45ec159c116063f
  10:     0x7f4bb29915c7 - rust_begin_unwind
  11:     0x7f4bb29460c3 - core::panicking::panic_fmt::h751a2d43b7aaf7e4
  12:     0x7f4bb3a1bb45 - <rustc_middle[bcb1ec240aa47d18]::ty::sty::Binder<rustc_middle[bcb1ec240aa47d18]::ty::PredicateKind>>::dummy
  13:     0x7f4bb393a815 - <rustc_hir_typeck[bc9466e1e0d1c1a1]::fn_ctxt::FnCtxt>::register_wf_obligation
  14:     0x7f4bb3939c50 - <rustc_hir_typeck[bc9466e1e0d1c1a1]::fn_ctxt::FnCtxt>::to_ty
  15:     0x7f4bb3b2d910 - <rustc_hir_typeck[bc9466e1e0d1c1a1]::gather_locals::GatherLocalsVisitor>::declare
  16:     0x7f4bb3b2df5f - <rustc_hir_typeck[bc9466e1e0d1c1a1]::gather_locals::GatherLocalsVisitor as rustc_hir[5fdcf93fb69859c1]::intravisit::Visitor>::visit_local
  17:     0x7f4bb3b2d084 - <rustc_hir_typeck[bc9466e1e0d1c1a1]::gather_locals::GatherLocalsVisitor as rustc_hir[5fdcf93fb69859c1]::intravisit::Visitor>::visit_block
  18:     0x7f4bb39fc95e - rustc_hir_typeck[bc9466e1e0d1c1a1]::typeck
  19:     0x7f4bb5218dd6 - rustc_query_system[67bb8d70703e5e0]::query::plumbing::try_execute_query::<rustc_query_impl[28bf92f7e9cf9953]::queries::typeck, rustc_query_impl[28bf92f7e9cf9953]::plumbing::QueryCtxt>
  20:     0x7f4bb4f1d9df - rustc_query_impl[28bf92f7e9cf9953]::get_query::typeck
  21:     0x7f4bb473cb0a - rustc_mir_build[9d9a80ededb6df87]::thir::cx::thir_body
  22:     0x7f4bb52308f0 - rustc_query_system[67bb8d70703e5e0]::query::plumbing::try_execute_query::<rustc_query_impl[28bf92f7e9cf9953]::queries::thir_body, rustc_query_impl[28bf92f7e9cf9953]::plumbing::QueryCtxt>
  23:     0x7f4bb4effeaf - rustc_query_impl[28bf92f7e9cf9953]::get_query::thir_body
  24:     0x7f4bb38d65be - rustc_ty_utils[9d90aa19451204b]::consts::thir_abstract_const
  25:     0x7f4bb5174f6c - rustc_query_system[67bb8d70703e5e0]::query::plumbing::try_execute_query::<rustc_query_impl[28bf92f7e9cf9953]::queries::thir_abstract_const, rustc_query_impl[28bf92f7e9cf9953]::plumbing::QueryCtxt>
  26:     0x7f4bb4f02d7e - rustc_query_impl[28bf92f7e9cf9953]::get_query::thir_abstract_const
  27:     0x7f4bb6002bf7 - rustc_middle[bcb1ec240aa47d18]::ty::query::query_get_at::<rustc_query_system[67bb8d70703e5e0]::query::caches::DefaultCache<rustc_span[1e4353f243e85ae]::def_id::DefId, rustc_middle[bcb1ec240aa47d18]::query::erase::Erased<[u8; 16usize]>>>
  28:     0x7f4bb5ffff4e - <rustc_middle[bcb1ec240aa47d18]::ty::context::TyCtxt>::bound_abstract_const
  29:     0x7f4bb61048be - <<rustc_middle[bcb1ec240aa47d18]::ty::context::TyCtxt>::expand_abstract_consts::Expander as rustc_type_ir[c475fa6bbe20e769]::fold::TypeFolder<rustc_middle[bcb1ec240aa47d18]::ty::context::TyCtxt>>::fold_const
  30:     0x7f4bb5babe36 - rustc_trait_selection[2608253ebd3b57b]::traits::const_evaluatable::is_const_evaluatable
  31:     0x7f4bb5c1c6f8 - <rustc_trait_selection[2608253ebd3b57b]::traits::fulfill::FulfillProcessor as rustc_data_structures[d5b45647156adf7c]::obligation_forest::ObligationProcessor>::process_obligation
  32:     0x7f4bb5da917c - <rustc_data_structures[d5b45647156adf7c]::obligation_forest::ObligationForest<rustc_trait_selection[2608253ebd3b57b]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[2608253ebd3b57b]::traits::fulfill::FulfillProcessor>
  33:     0x7f4bb5c14a94 - <rustc_trait_selection[2608253ebd3b57b]::traits::fulfill::FulfillmentContext as rustc_infer[f2a32f942b1e93c4]::traits::engine::TraitEngine>::select_where_possible
  34:     0x7f4bb5b9c702 - <dyn rustc_infer[f2a32f942b1e93c4]::traits::engine::TraitEngine as rustc_infer[f2a32f942b1e93c4]::traits::engine::TraitEngineExt>::select_all_or_error
  35:     0x7f4bb5d139bd - <rustc_trait_selection[2608253ebd3b57b]::traits::engine::ObligationCtxt>::select_all_or_error
  36:     0x7f4bb3c50e93 - rustc_hir_analysis[a292bf8fb7a5e941]::check::wfcheck::check_item_fn
  37:     0x7f4bb3c47ed6 - rustc_hir_analysis[a292bf8fb7a5e941]::check::wfcheck::check_well_formed
  38:     0x7f4bb513fe17 - rustc_query_system[67bb8d70703e5e0]::query::plumbing::try_execute_query::<rustc_query_impl[28bf92f7e9cf9953]::queries::check_well_formed, rustc_query_impl[28bf92f7e9cf9953]::plumbing::QueryCtxt>
  39:     0x7f4bb4f41f47 - rustc_query_impl[28bf92f7e9cf9953]::get_query::check_well_formed
  40:     0x7f4bb3d13acd - std[bb93a438393cee13]::panicking::try::<(), core[1754ff5e11a74914]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[d5b45647156adf7c]::sync::par_for_each_in<&[rustc_hir[5fdcf93fb69859c1]::hir::ItemId], <rustc_middle[bcb1ec240aa47d18]::hir::ModuleItems>::par_items<rustc_hir_analysis[a292bf8fb7a5e941]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  41:     0x7f4bb3c970fd - rustc_data_structures[d5b45647156adf7c]::sync::par_for_each_in::<&[rustc_hir[5fdcf93fb69859c1]::hir::ItemId], <rustc_middle[bcb1ec240aa47d18]::hir::ModuleItems>::par_items<rustc_hir_analysis[a292bf8fb7a5e941]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  42:     0x7f4bb3c5374c - rustc_hir_analysis[a292bf8fb7a5e941]::check::wfcheck::check_mod_type_wf
  43:     0x7f4bb513eb9f - rustc_query_system[67bb8d70703e5e0]::query::plumbing::try_execute_query::<rustc_query_impl[28bf92f7e9cf9953]::queries::check_mod_type_wf, rustc_query_impl[28bf92f7e9cf9953]::plumbing::QueryCtxt>
  44:     0x7f4bb4f1c607 - rustc_query_impl[28bf92f7e9cf9953]::get_query::check_mod_type_wf
  45:     0x7f4bb3d13bfd - std[bb93a438393cee13]::panicking::try::<(), core[1754ff5e11a74914]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[d5b45647156adf7c]::sync::par_for_each_in<&[rustc_hir[5fdcf93fb69859c1]::hir_id::OwnerId], <rustc_middle[bcb1ec240aa47d18]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[a292bf8fb7a5e941]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  46:     0x7f4bb3c9726d - rustc_data_structures[d5b45647156adf7c]::sync::par_for_each_in::<&[rustc_hir[5fdcf93fb69859c1]::hir_id::OwnerId], <rustc_middle[bcb1ec240aa47d18]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[a292bf8fb7a5e941]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  47:     0x7f4bb3d69b39 - rustc_hir_analysis[a292bf8fb7a5e941]::check_crate
  48:     0x7f4bb3531cc9 - rustc_interface[6bd45d958f65e896]::passes::analysis
  49:     0x7f4bb521d679 - rustc_query_system[67bb8d70703e5e0]::query::plumbing::try_execute_query::<rustc_query_impl[28bf92f7e9cf9953]::queries::analysis, rustc_query_impl[28bf92f7e9cf9953]::plumbing::QueryCtxt>
  50:     0x7f4bb4ef9b5f - rustc_query_impl[28bf92f7e9cf9953]::get_query::analysis
  51:     0x7f4bb346f02d - <rustc_middle[bcb1ec240aa47d18]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[311026437090329a]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>>
  52:     0x7f4bb349d4f2 - <rustc_interface[6bd45d958f65e896]::interface::Compiler>::enter::<rustc_driver_impl[311026437090329a]::run_compiler::{closure#1}::{closure#2}, core[1754ff5e11a74914]::result::Result<core[1754ff5e11a74914]::option::Option<rustc_interface[6bd45d958f65e896]::queries::Linker>, rustc_span[1e4353f243e85ae]::ErrorGuaranteed>>
  53:     0x7f4bb34680d0 - rustc_span[1e4353f243e85ae]::set_source_map::<core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>, rustc_interface[6bd45d958f65e896]::interface::run_compiler<core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>, rustc_driver_impl[311026437090329a]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  54:     0x7f4bb3461379 - <scoped_tls[f7fb0bc843e0be75]::ScopedKey<rustc_span[1e4353f243e85ae]::SessionGlobals>>::set::<rustc_interface[6bd45d958f65e896]::interface::run_compiler<core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>, rustc_driver_impl[311026437090329a]::run_compiler::{closure#1}>::{closure#0}, core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>>
  55:     0x7f4bb3477bc6 - std[bb93a438393cee13]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6bd45d958f65e896]::util::run_in_thread_pool_with_globals<rustc_interface[6bd45d958f65e896]::interface::run_compiler<core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>, rustc_driver_impl[311026437090329a]::run_compiler::{closure#1}>::{closure#0}, core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>>
  56:     0x7f4bb34b7b76 - std[bb93a438393cee13]::panicking::try::<core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>, core[1754ff5e11a74914]::panic::unwind_safe::AssertUnwindSafe<<std[bb93a438393cee13]::thread::Builder>::spawn_unchecked_<rustc_interface[6bd45d958f65e896]::util::run_in_thread_pool_with_globals<rustc_interface[6bd45d958f65e896]::interface::run_compiler<core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>, rustc_driver_impl[311026437090329a]::run_compiler::{closure#1}>::{closure#0}, core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  57:     0x7f4bb34664ad - <<std[bb93a438393cee13]::thread::Builder>::spawn_unchecked_<rustc_interface[6bd45d958f65e896]::util::run_in_thread_pool_with_globals<rustc_interface[6bd45d958f65e896]::interface::run_compiler<core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>, rustc_driver_impl[311026437090329a]::run_compiler::{closure#1}>::{closure#0}, core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1754ff5e11a74914]::result::Result<(), rustc_span[1e4353f243e85ae]::ErrorGuaranteed>>::{closure#1} as core[1754ff5e11a74914]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  58:     0x7f4bb299e49e - std::sys::unix::thread::Thread::new::thread_start::h5470a0151f04427f
  59:     0x7f4bb2738b43 - <unknown>
  60:     0x7f4bb27caa00 - <unknown>
  61:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (1c19446d0 2023-05-04) running on x86_64-unknown-linux-gnu

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



