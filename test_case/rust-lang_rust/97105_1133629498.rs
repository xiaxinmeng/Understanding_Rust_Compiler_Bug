plain
.........i.....ii....................................................................... 1760/13118
........................................................................................ 1848/13118
...........................................................i............................ 1936/13118
........................................................................................ 2024/13118
.................................F...............F..........F........................... 2112/13118
........................................................................................ 2200/13118
.................F.............................................FFF...................... 2288/13118
........................................................................................ 2464/13118
........................................................................................ 2552/13118
........................................................................................ 2640/13118
........................................................................................ 2728/13118
---
failures:

---- [ui] src/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'internal error: entered unreachable code', compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:107:46
stack backtrace:
   0:     0x7f77c0d3a97c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5f278ce3b9a63834
   1:     0x7f77c0da0fd8 - core::fmt::write::hfaaf3f7e811d7d79
   2:     0x7f77c0d2a761 - std::io::Write::write_fmt::h695784041433cf41
   3:     0x7f77c0d3d9ce - std::panicking::default_hook::{{closure}}::h24ec1953f381a8f4
   4:     0x7f77c0d3d5fc - std::panicking::default_hook::hb6535462245f12ca
   5:     0x7f77c18d3f11 - rustc_driver[eccc9f23e5d4e674]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f77c0d3e22e - std::panicking::rust_panic_with_hook::h67a5fa7ec0514204
   7:     0x7f77c0d3dfe9 - std::panicking::begin_panic_handler::{{closure}}::h4d41dac866db9bba
   8:     0x7f77c0d3ae94 - std::sys_common::backtrace::__rust_end_short_backtrace::h1a7b7284f6dba6e7
   9:     0x7f77c0d3dd09 - rust_begin_unwind
  10:     0x7f77c0cf2073 - core::panicking::panic_fmt::hca9d2d765596d598
  11:     0x7f77c0cf1f3d - core::panicking::panic::h8a02272c1a26df46
  12:     0x7f77c3f15a0d - rustc_trait_selection[9563980304af3adb]::traits::const_evaluatable::is_const_evaluatable
  13:     0x7f77c3e5ed21 - <rustc_trait_selection[9563980304af3adb]::traits::fulfill::FulfillProcessor>::process_changed_obligations
  14:     0x7f77c3ebfc7e - <rustc_data_structures[67476deb84584d5d]::obligation_forest::ObligationForest<rustc_trait_selection[9563980304af3adb]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[9563980304af3adb]::traits::fulfill::FulfillProcessor, rustc_data_structures[67476deb84584d5d]::obligation_forest::Outcome<rustc_trait_selection[9563980304af3adb]::traits::fulfill::PendingPredicateObligation, rustc_infer[79f42d7d4b1ef223]::traits::FulfillmentErrorCode>>
  15:     0x7f77c3e5d61b - <rustc_trait_selection[9563980304af3adb]::traits::fulfill::FulfillmentContext as rustc_infer[79f42d7d4b1ef223]::traits::engine::TraitEngine>::select_where_possible
  16:     0x7f77c20b9cad - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::resolve_vars_with_obligations
  17:     0x7f77c20bf14a - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::structurally_resolved_type
  18:     0x7f77c2099000 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_call
  19:     0x7f77c210496e - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  20:     0x7f77c20ae7a7 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  21:     0x7f77c2103799 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  22:     0x7f77c210ba20 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  23:     0x7f77c20ae7a7 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  24:     0x7f77c2103799 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  25:     0x7f77c20c830b - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_stmt
  26:     0x7f77c20c88f4 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  27:     0x7f77c2104c1a - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  28:     0x7f77c20ae7a7 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  29:     0x7f77c2103799 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  30:     0x7f77c20afba7 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_return_expr
  31:     0x7f77c2410c82 - rustc_typeck[4a3bf33c0b57c62a]::check::check::check_fn
  32:     0x7f77c22b912e - <rustc_infer[79f42d7d4b1ef223]::infer::InferCtxtBuilder>::enter::<&rustc_middle[bbd1842e24523309]::ty::context::TypeckResults, <rustc_typeck[4a3bf33c0b57c62a]::check::inherited::InheritedBuilder>::enter<rustc_typeck[4a3bf33c0b57c62a]::check::typeck_with_fallback<rustc_typeck[4a3bf33c0b57c62a]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[bbd1842e24523309]::ty::context::TypeckResults>::{closure#0}>
  33:     0x7f77c23d6e3e - <rustc_typeck[4a3bf33c0b57c62a]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[4a3bf33c0b57c62a]::check::typeck_with_fallback<rustc_typeck[4a3bf33c0b57c62a]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[bbd1842e24523309]::ty::context::TypeckResults>
  34:     0x7f77c21deafd - rustc_typeck[4a3bf33c0b57c62a]::check::typeck
  35:     0x7f77c33148e6 - rustc_query_system[5113d42b3843a911]::query::plumbing::try_execute_query::<rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt, rustc_query_system[5113d42b3843a911]::query::caches::DefaultCache<rustc_span[95f14e4ccbe972d3]::def_id::LocalDefId, &rustc_middle[bbd1842e24523309]::ty::context::TypeckResults>>
  36:     0x7f77c342ff27 - rustc_query_system[5113d42b3843a911]::query::plumbing::get_query::<rustc_query_impl[25ec45a1d1991b1c]::queries::typeck, rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt>
  37:     0x7f77c2fa84c4 - <rustc_query_impl[25ec45a1d1991b1c]::Queries as rustc_middle[bbd1842e24523309]::ty::query::QueryEngine>::typeck
  38:     0x7f77c23e7b76 - rustc_typeck[4a3bf33c0b57c62a]::collect::type_of::opt_const_param_of
  39:     0x7f77c330671a - rustc_query_system[5113d42b3843a911]::query::plumbing::try_execute_query::<rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt, rustc_query_system[5113d42b3843a911]::query::caches::DefaultCache<rustc_span[95f14e4ccbe972d3]::def_id::LocalDefId, core[5e9b674adaad8ecd]::option::Option<rustc_span[95f14e4ccbe972d3]::def_id::DefId>>>
  40:     0x7f77c33f37fe - rustc_query_system[5113d42b3843a911]::query::plumbing::get_query::<rustc_query_impl[25ec45a1d1991b1c]::queries::opt_const_param_of, rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt>
  41:     0x7f77c2f8ae64 - <rustc_query_impl[25ec45a1d1991b1c]::Queries as rustc_middle[bbd1842e24523309]::ty::query::QueryEngine>::opt_const_param_of
  42:     0x7f77c21de4b0 - rustc_typeck[4a3bf33c0b57c62a]::check::typeck
  43:     0x7f77c33148e6 - rustc_query_system[5113d42b3843a911]::query::plumbing::try_execute_query::<rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt, rustc_query_system[5113d42b3843a911]::query::caches::DefaultCache<rustc_span[95f14e4ccbe972d3]::def_id::LocalDefId, &rustc_middle[bbd1842e24523309]::ty::context::TypeckResults>>
  44:     0x7f77c342ff27 - rustc_query_system[5113d42b3843a911]::query::plumbing::get_query::<rustc_query_impl[25ec45a1d1991b1c]::queries::typeck, rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt>
  45:     0x7f77c2fa84c4 - <rustc_query_impl[25ec45a1d1991b1c]::Queries as rustc_middle[bbd1842e24523309]::ty::query::QueryEngine>::typeck
  46:     0x7f77c237bc5b - <rustc_middle[bbd1842e24523309]::hir::map::Map>::par_body_owners::<rustc_typeck[4a3bf33c0b57c62a]::check::typeck_item_bodies::{closure#0}>
  47:     0x7f77c21e37ed - rustc_typeck[4a3bf33c0b57c62a]::check::typeck_item_bodies
  48:     0x7f77c335c37a - rustc_query_system[5113d42b3843a911]::query::plumbing::try_execute_query::<rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt, rustc_query_system[5113d42b3843a911]::query::caches::DefaultCache<(), ()>>
  49:     0x7f77c33f3a55 - rustc_query_system[5113d42b3843a911]::query::plumbing::get_query::<rustc_query_impl[25ec45a1d1991b1c]::queries::typeck_item_bodies, rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt>
  50:     0x7f77c2fa7f6e - <rustc_query_impl[25ec45a1d1991b1c]::Queries as rustc_middle[bbd1842e24523309]::ty::query::QueryEngine>::typeck_item_bodies
  51:     0x7f77c22437fa - <rustc_session[2266dbf95b3f99af]::session::Session>::time::<(), rustc_typeck[4a3bf33c0b57c62a]::check_crate::{closure#7}>
  52:     0x7f77c223fdb8 - rustc_typeck[4a3bf33c0b57c62a]::check_crate
  53:     0x7f77c19a8691 - rustc_interface[2c90a0c0bfdfedc]::passes::analysis
  54:     0x7f77c3350a3e - rustc_query_system[5113d42b3843a911]::query::plumbing::try_execute_query::<rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt, rustc_query_system[5113d42b3843a911]::query::caches::DefaultCache<(), core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>>
  55:     0x7f77c3430302 - rustc_query_system[5113d42b3843a911]::query::plumbing::get_query::<rustc_query_impl[25ec45a1d1991b1c]::queries::analysis, rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt>
  56:     0x7f77c2f8be7e - <rustc_query_impl[25ec45a1d1991b1c]::Queries as rustc_middle[bbd1842e24523309]::ty::query::QueryEngine>::analysis
  57:     0x7f77c18b62f4 - <rustc_interface[2c90a0c0bfdfedc]::passes::QueryContext>::enter::<rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>
  58:     0x7f77c186260e - <rustc_interface[2c90a0c0bfdfedc]::interface::Compiler>::enter::<rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}::{closure#2}, core[5e9b674adaad8ecd]::result::Result<core[5e9b674adaad8ecd]::option::Option<rustc_interface[2c90a0c0bfdfedc]::queries::Linker>, rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>
  59:     0x7f77c184430b - rustc_span[95f14e4ccbe972d3]::with_source_map::<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_interface[2c90a0c0bfdfedc]::interface::create_compiler_and_run<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}>::{closure#1}>
  60:     0x7f77c18637a9 - <scoped_tls[5fdc27eea9245943]::ScopedKey<rustc_span[95f14e4ccbe972d3]::SessionGlobals>>::set::<rustc_interface[2c90a0c0bfdfedc]::interface::run_compiler<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>
  61:     0x7f77c18bf239 - std[cf1d69d5cbb98166]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2c90a0c0bfdfedc]::util::run_in_thread_pool_with_globals<rustc_interface[2c90a0c0bfdfedc]::interface::run_compiler<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>
  62:     0x7f77c1877f01 - std[cf1d69d5cbb98166]::panicking::try::<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, core[5e9b674adaad8ecd]::panic::unwind_safe::AssertUnwindSafe<<std[cf1d69d5cbb98166]::thread::Builder>::spawn_unchecked_<rustc_interface[2c90a0c0bfdfedc]::util::run_in_thread_pool_with_globals<rustc_interface[2c90a0c0bfdfedc]::interface::run_compiler<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  63:     0x7f77c18b9ea2 - <<std[cf1d69d5cbb98166]::thread::Builder>::spawn_unchecked_<rustc_interface[2c90a0c0bfdfedc]::util::run_in_thread_pool_with_globals<rustc_interface[2c90a0c0bfdfedc]::interface::run_compiler<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>::{closure#1} as core[5e9b674adaad8ecd]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  64:     0x7f77c0d49603 - std::sys::unix::thread::Thread::new::thread_start::h4a439a3f4cae6a0e
  65:     0x7f77bb29b609 - start_thread
  66:     0x7f77c0bae133 - clone
  67:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (a960aa9c6 2022-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [opt_const_param_of] computing the optional const parameter of `main::{constant#0}`
#2 [typeck] type-checking `main::{constant#0}`
#3 [typeck_item_bodies] type-checking all item bodies
#4 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] src/test/ui/const-generics/generic_const_exprs/infer-too-generic.rs stdout ----
---- [ui] src/test/ui/const-generics/generic_const_exprs/infer-too-generic.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/infer-too-generic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/infer-too-generic/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/infer-too-generic/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'internal error: entered unreachable code', compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:107:46
stack backtrace:
   0:     0x7efe6a7f697c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5f278ce3b9a63834
   1:     0x7efe6a85cfd8 - core::fmt::write::hfaaf3f7e811d7d79
   2:     0x7efe6a7e6761 - std::io::Write::write_fmt::h695784041433cf41
   3:     0x7efe6a7f99ce - std::panicking::default_hook::{{closure}}::h24ec1953f381a8f4
   4:     0x7efe6a7f95fc - std::panicking::default_hook::hb6535462245f12ca
   5:     0x7efe6b38ff11 - rustc_driver[eccc9f23e5d4e674]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7efe6a7fa22e - std::panicking::rust_panic_with_hook::h67a5fa7ec0514204
   7:     0x7efe6a7f9fe9 - std::panicking::begin_panic_handler::{{closure}}::h4d41dac866db9bba
   8:     0x7efe6a7f6e94 - std::sys_common::backtrace::__rust_end_short_backtrace::h1a7b7284f6dba6e7
   9:     0x7efe6a7f9d09 - rust_begin_unwind
  10:     0x7efe6a7ae073 - core::panicking::panic_fmt::hca9d2d765596d598
  11:     0x7efe6a7adf3d - core::panicking::panic::h8a02272c1a26df46
  12:     0x7efe6d9d1a0d - rustc_trait_selection[9563980304af3adb]::traits::const_evaluatable::is_const_evaluatable
  13:     0x7efe6d91ad21 - <rustc_trait_selection[9563980304af3adb]::traits::fulfill::FulfillProcessor>::process_changed_obligations
  14:     0x7efe6d97bc7e - <rustc_data_structures[67476deb84584d5d]::obligation_forest::ObligationForest<rustc_trait_selection[9563980304af3adb]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[9563980304af3adb]::traits::fulfill::FulfillProcessor, rustc_data_structures[67476deb84584d5d]::obligation_forest::Outcome<rustc_trait_selection[9563980304af3adb]::traits::fulfill::PendingPredicateObligation, rustc_infer[79f42d7d4b1ef223]::traits::FulfillmentErrorCode>>
  15:     0x7efe6d91961b - <rustc_trait_selection[9563980304af3adb]::traits::fulfill::FulfillmentContext as rustc_infer[79f42d7d4b1ef223]::traits::engine::TraitEngine>::select_where_possible
  16:     0x7efe6bb7c427 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_argument_types
  17:     0x7efe6bb57a75 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::confirm_builtin_call
  18:     0x7efe6bb55afb - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_call
  19:     0x7efe6bbc096e - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  20:     0x7efe6bb6a7a7 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  21:     0x7efe6bbbf799 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  22:     0x7efe6bb83fa5 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_decl
  23:     0x7efe6bb841b3 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_stmt
  24:     0x7efe6bb848f4 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  25:     0x7efe6bbc0c1a - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  26:     0x7efe6bb6a7a7 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  27:     0x7efe6bbbf799 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  28:     0x7efe6bb6bba7 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_return_expr
  29:     0x7efe6beccc82 - rustc_typeck[4a3bf33c0b57c62a]::check::check::check_fn
  30:     0x7efe6bd7512e - <rustc_infer[79f42d7d4b1ef223]::infer::InferCtxtBuilder>::enter::<&rustc_middle[bbd1842e24523309]::ty::context::TypeckResults, <rustc_typeck[4a3bf33c0b57c62a]::check::inherited::InheritedBuilder>::enter<rustc_typeck[4a3bf33c0b57c62a]::check::typeck_with_fallback<rustc_typeck[4a3bf33c0b57c62a]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[bbd1842e24523309]::ty::context::TypeckResults>::{closure#0}>
  31:     0x7efe6be92e3e - <rustc_typeck[4a3bf33c0b57c62a]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[4a3bf33c0b57c62a]::check::typeck_with_fallback<rustc_typeck[4a3bf33c0b57c62a]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[bbd1842e24523309]::ty::context::TypeckResults>
  32:     0x7efe6bc9aafd - rustc_typeck[4a3bf33c0b57c62a]::check::typeck
  33:     0x7efe6cdd08e6 - rustc_query_system[5113d42b3843a911]::query::plumbing::try_execute_query::<rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt, rustc_query_system[5113d42b3843a911]::query::caches::DefaultCache<rustc_span[95f14e4ccbe972d3]::def_id::LocalDefId, &rustc_middle[bbd1842e24523309]::ty::context::TypeckResults>>
  34:     0x7efe6ceebf27 - rustc_query_system[5113d42b3843a911]::query::plumbing::get_query::<rustc_query_impl[25ec45a1d1991b1c]::queries::typeck, rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt>
  35:     0x7efe6ca644c4 - <rustc_query_impl[25ec45a1d1991b1c]::Queries as rustc_middle[bbd1842e24523309]::ty::query::QueryEngine>::typeck
  36:     0x7efe6be37c5b - <rustc_middle[bbd1842e24523309]::hir::map::Map>::par_body_owners::<rustc_typeck[4a3bf33c0b57c62a]::check::typeck_item_bodies::{closure#0}>
  37:     0x7efe6bc9f7ed - rustc_typeck[4a3bf33c0b57c62a]::check::typeck_item_bodies
  38:     0x7efe6ce1837a - rustc_query_system[5113d42b3843a911]::query::plumbing::try_execute_query::<rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt, rustc_query_system[5113d42b3843a911]::query::caches::DefaultCache<(), ()>>
  39:     0x7efe6ceafa55 - rustc_query_system[5113d42b3843a911]::query::plumbing::get_query::<rustc_query_impl[25ec45a1d1991b1c]::queries::typeck_item_bodies, rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt>
  40:     0x7efe6ca63f6e - <rustc_query_impl[25ec45a1d1991b1c]::Queries as rustc_middle[bbd1842e24523309]::ty::query::QueryEngine>::typeck_item_bodies
  41:     0x7efe6bcff7fa - <rustc_session[2266dbf95b3f99af]::session::Session>::time::<(), rustc_typeck[4a3bf33c0b57c62a]::check_crate::{closure#7}>
  42:     0x7efe6bcfbdb8 - rustc_typeck[4a3bf33c0b57c62a]::check_crate
  43:     0x7efe6b464691 - rustc_interface[2c90a0c0bfdfedc]::passes::analysis
  44:     0x7efe6ce0ca3e - rustc_query_system[5113d42b3843a911]::query::plumbing::try_execute_query::<rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt, rustc_query_system[5113d42b3843a911]::query::caches::DefaultCache<(), core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>>
  45:     0x7efe6ceec302 - rustc_query_system[5113d42b3843a911]::query::plumbing::get_query::<rustc_query_impl[25ec45a1d1991b1c]::queries::analysis, rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt>
  46:     0x7efe6ca47e7e - <rustc_query_impl[25ec45a1d1991b1c]::Queries as rustc_middle[bbd1842e24523309]::ty::query::QueryEngine>::analysis
  47:     0x7efe6b3722f4 - <rustc_interface[2c90a0c0bfdfedc]::passes::QueryContext>::enter::<rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>
  48:     0x7efe6b31e60e - <rustc_interface[2c90a0c0bfdfedc]::interface::Compiler>::enter::<rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}::{closure#2}, core[5e9b674adaad8ecd]::result::Result<core[5e9b674adaad8ecd]::option::Option<rustc_interface[2c90a0c0bfdfedc]::queries::Linker>, rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>
  49:     0x7efe6b30030b - rustc_span[95f14e4ccbe972d3]::with_source_map::<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_interface[2c90a0c0bfdfedc]::interface::create_compiler_and_run<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}>::{closure#1}>
  50:     0x7efe6b31f7a9 - <scoped_tls[5fdc27eea9245943]::ScopedKey<rustc_span[95f14e4ccbe972d3]::SessionGlobals>>::set::<rustc_interface[2c90a0c0bfdfedc]::interface::run_compiler<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>
  51:     0x7efe6b37b239 - std[cf1d69d5cbb98166]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2c90a0c0bfdfedc]::util::run_in_thread_pool_with_globals<rustc_interface[2c90a0c0bfdfedc]::interface::run_compiler<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>
  52:     0x7efe6b333f01 - std[cf1d69d5cbb98166]::panicking::try::<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, core[5e9b674adaad8ecd]::panic::unwind_safe::AssertUnwindSafe<<std[cf1d69d5cbb98166]::thread::Builder>::spawn_unchecked_<rustc_interface[2c90a0c0bfdfedc]::util::run_in_thread_pool_with_globals<rustc_interface[2c90a0c0bfdfedc]::interface::run_compiler<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  53:     0x7efe6b375ea2 - <<std[cf1d69d5cbb98166]::thread::Builder>::spawn_unchecked_<rustc_interface[2c90a0c0bfdfedc]::util::run_in_thread_pool_with_globals<rustc_interface[2c90a0c0bfdfedc]::interface::run_compiler<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>::{closure#1} as core[5e9b674adaad8ecd]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  54:     0x7efe6a805603 - std::sys::unix::thread::Thread::new::thread_start::h4a439a3f4cae6a0e
  55:     0x7efe64d57609 - start_thread
  56:     0x7efe6a66a133 - clone
  57:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (a960aa9c6 2022-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-76595.rs stdout ----
---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-76595.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-76595.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-76595" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-76595/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0107]: this function takes 2 generic arguments but 1 generic argument was supplied
   |
LL |     test::<2>();
LL |     test::<2>();
   |     ^^^^   - supplied 1 generic argument
   |     expected 2 generic arguments
   |
   |
note: function defined here, with 2 generic parameters: `T`, `P`
   |
   |
LL | fn test<T, const P: usize>() where Bool<{core::mem::size_of::<T>() > 4}>: True {
   |    ^^^^ -  --------------
help: add missing generic argument
   |
LL |     test::<2, P>();

thread 'rustc' panicked at 'internal error: entered unreachable code', compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:107:46
stack backtrace:
   0:     0x7fd559e8f97c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5f278ce3b9a63834
   0:     0x7fd559e8f97c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5f278ce3b9a63834
   1:     0x7fd559ef5fd8 - core::fmt::write::hfaaf3f7e811d7d79
   2:     0x7fd559e7f761 - std::io::Write::write_fmt::h695784041433cf41
   3:     0x7fd559e929ce - std::panicking::default_hook::{{closure}}::h24ec1953f381a8f4
   4:     0x7fd559e925fc - std::panicking::default_hook::hb6535462245f12ca
   5:     0x7fd55aa28f11 - rustc_driver[eccc9f23e5d4e674]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fd559e9322e - std::panicking::rust_panic_with_hook::h67a5fa7ec0514204
   7:     0x7fd559e92fe9 - std::panicking::begin_panic_handler::{{closure}}::h4d41dac866db9bba
   8:     0x7fd559e8fe94 - std::sys_common::backtrace::__rust_end_short_backtrace::h1a7b7284f6dba6e7
   9:     0x7fd559e92d09 - rust_begin_unwind
  10:     0x7fd559e47073 - core::panicking::panic_fmt::hca9d2d765596d598
  11:     0x7fd559e46f3d - core::panicking::panic::h8a02272c1a26df46
  12:     0x7fd55d06aa0d - rustc_trait_selection[9563980304af3adb]::traits::const_evaluatable::is_const_evaluatable
  13:     0x7fd55cfb3d21 - <rustc_trait_selection[9563980304af3adb]::traits::fulfill::FulfillProcessor>::process_changed_obligations
  14:     0x7fd55d014c7e - <rustc_data_structures[67476deb84584d5d]::obligation_forest::ObligationForest<rustc_trait_selection[9563980304af3adb]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[9563980304af3adb]::traits::fulfill::FulfillProcessor, rustc_data_structures[67476deb84584d5d]::obligation_forest::Outcome<rustc_trait_selection[9563980304af3adb]::traits::fulfill::PendingPredicateObligation, rustc_infer[79f42d7d4b1ef223]::traits::FulfillmentErrorCode>>
  15:     0x7fd55cfb261b - <rustc_trait_selection[9563980304af3adb]::traits::fulfill::FulfillmentContext as rustc_infer[79f42d7d4b1ef223]::traits::engine::TraitEngine>::select_where_possible
  16:     0x7fd55b20c26d - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::type_inference_fallback
  17:     0x7fd55b40e56b - <rustc_infer[79f42d7d4b1ef223]::infer::InferCtxtBuilder>::enter::<&rustc_middle[bbd1842e24523309]::ty::context::TypeckResults, <rustc_typeck[4a3bf33c0b57c62a]::check::inherited::InheritedBuilder>::enter<rustc_typeck[4a3bf33c0b57c62a]::check::typeck_with_fallback<rustc_typeck[4a3bf33c0b57c62a]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[bbd1842e24523309]::ty::context::TypeckResults>::{closure#0}>
  18:     0x7fd55b52be3e - <rustc_typeck[4a3bf33c0b57c62a]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[4a3bf33c0b57c62a]::check::typeck_with_fallback<rustc_typeck[4a3bf33c0b57c62a]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[bbd1842e24523309]::ty::context::TypeckResults>
  19:     0x7fd55b333afd - rustc_typeck[4a3bf33c0b57c62a]::check::typeck
  20:     0x7fd55c4698e6 - rustc_query_system[5113d42b3843a911]::query::plumbing::try_execute_query::<rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt, rustc_query_system[5113d42b3843a911]::query::caches::DefaultCache<rustc_span[95f14e4ccbe972d3]::def_id::LocalDefId, &rustc_middle[bbd1842e24523309]::ty::context::TypeckResults>>
  21:     0x7fd55c584f27 - rustc_query_system[5113d42b3843a911]::query::plumbing::get_query::<rustc_query_impl[25ec45a1d1991b1c]::queries::typeck, rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt>
  22:     0x7fd55c0fd4c4 - <rustc_query_impl[25ec45a1d1991b1c]::Queries as rustc_middle[bbd1842e24523309]::ty::query::QueryEngine>::typeck
  23:     0x7fd55b53cb76 - rustc_typeck[4a3bf33c0b57c62a]::collect::type_of::opt_const_param_of
  24:     0x7fd55c45b71a - rustc_query_system[5113d42b3843a911]::query::plumbing::try_execute_query::<rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt, rustc_query_system[5113d42b3843a911]::query::caches::DefaultCache<rustc_span[95f14e4ccbe972d3]::def_id::LocalDefId, core[5e9b674adaad8ecd]::option::Option<rustc_span[95f14e4ccbe972d3]::def_id::DefId>>>
  25:     0x7fd55c5487fe - rustc_query_system[5113d42b3843a911]::query::plumbing::get_query::<rustc_query_impl[25ec45a1d1991b1c]::queries::opt_const_param_of, rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt>
  26:     0x7fd55c0dfe64 - <rustc_query_impl[25ec45a1d1991b1c]::Queries as rustc_middle[bbd1842e24523309]::ty::query::QueryEngine>::opt_const_param_of
  27:     0x7fd55b3334b0 - rustc_typeck[4a3bf33c0b57c62a]::check::typeck
  28:     0x7fd55c4698e6 - rustc_query_system[5113d42b3843a911]::query::plumbing::try_execute_query::<rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt, rustc_query_system[5113d42b3843a911]::query::caches::DefaultCache<rustc_span[95f14e4ccbe972d3]::def_id::LocalDefId, &rustc_middle[bbd1842e24523309]::ty::context::TypeckResults>>
  29:     0x7fd55c584f27 - rustc_query_system[5113d42b3843a911]::query::plumbing::get_query::<rustc_query_impl[25ec45a1d1991b1c]::queries::typeck, rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt>
  30:     0x7fd55c0fd4c4 - <rustc_query_impl[25ec45a1d1991b1c]::Queries as rustc_middle[bbd1842e24523309]::ty::query::QueryEngine>::typeck
  31:     0x7fd55b4d0c5b - <rustc_middle[bbd1842e24523309]::hir::map::Map>::par_body_owners::<rustc_typeck[4a3bf33c0b57c62a]::check::typeck_item_bodies::{closure#0}>
  32:     0x7fd55b3387ed - rustc_typeck[4a3bf33c0b57c62a]::check::typeck_item_bodies
  33:     0x7fd55c4b137a - rustc_query_system[5113d42b3843a911]::query::plumbing::try_execute_query::<rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt, rustc_query_system[5113d42b3843a911]::query::caches::DefaultCache<(), ()>>
  34:     0x7fd55c548a55 - rustc_query_system[5113d42b3843a911]::query::plumbing::get_query::<rustc_query_impl[25ec45a1d1991b1c]::queries::typeck_item_bodies, rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt>
  35:     0x7fd55c0fcf6e - <rustc_query_impl[25ec45a1d1991b1c]::Queries as rustc_middle[bbd1842e24523309]::ty::query::QueryEngine>::typeck_item_bodies
  36:     0x7fd55b3987fa - <rustc_session[2266dbf95b3f99af]::session::Session>::time::<(), rustc_typeck[4a3bf33c0b57c62a]::check_crate::{closure#7}>
  37:     0x7fd55b394db8 - rustc_typeck[4a3bf33c0b57c62a]::check_crate
  38:     0x7fd55aafd691 - rustc_interface[2c90a0c0bfdfedc]::passes::analysis
  39:     0x7fd55c4a5a3e - rustc_query_system[5113d42b3843a911]::query::plumbing::try_execute_query::<rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt, rustc_query_system[5113d42b3843a911]::query::caches::DefaultCache<(), core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>>
  40:     0x7fd55c585302 - rustc_query_system[5113d42b3843a911]::query::plumbing::get_query::<rustc_query_impl[25ec45a1d1991b1c]::queries::analysis, rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt>
  41:     0x7fd55c0e0e7e - <rustc_query_impl[25ec45a1d1991b1c]::Queries as rustc_middle[bbd1842e24523309]::ty::query::QueryEngine>::analysis
  42:     0x7fd55aa0b2f4 - <rustc_interface[2c90a0c0bfdfedc]::passes::QueryContext>::enter::<rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>
  43:     0x7fd55a9b760e - <rustc_interface[2c90a0c0bfdfedc]::interface::Compiler>::enter::<rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}::{closure#2}, core[5e9b674adaad8ecd]::result::Result<core[5e9b674adaad8ecd]::option::Option<rustc_interface[2c90a0c0bfdfedc]::queries::Linker>, rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>
  44:     0x7fd55a99930b - rustc_span[95f14e4ccbe972d3]::with_source_map::<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_interface[2c90a0c0bfdfedc]::interface::create_compiler_and_run<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}>::{closure#1}>
  45:     0x7fd55a9b87a9 - <scoped_tls[5fdc27eea9245943]::ScopedKey<rustc_span[95f14e4ccbe972d3]::SessionGlobals>>::set::<rustc_interface[2c90a0c0bfdfedc]::interface::run_compiler<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>
  46:     0x7fd55aa14239 - std[cf1d69d5cbb98166]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2c90a0c0bfdfedc]::util::run_in_thread_pool_with_globals<rustc_interface[2c90a0c0bfdfedc]::interface::run_compiler<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>
  47:     0x7fd55a9ccf01 - std[cf1d69d5cbb98166]::panicking::try::<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, core[5e9b674adaad8ecd]::panic::unwind_safe::AssertUnwindSafe<<std[cf1d69d5cbb98166]::thread::Builder>::spawn_unchecked_<rustc_interface[2c90a0c0bfdfedc]::util::run_in_thread_pool_with_globals<rustc_interface[2c90a0c0bfdfedc]::interface::run_compiler<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  48:     0x7fd55aa0eea2 - <<std[cf1d69d5cbb98166]::thread::Builder>::spawn_unchecked_<rustc_interface[2c90a0c0bfdfedc]::util::run_in_thread_pool_with_globals<rustc_interface[2c90a0c0bfdfedc]::interface::run_compiler<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>::{closure#1} as core[5e9b674adaad8ecd]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  49:     0x7fd559e9e603 - std::sys::unix::thread::Thread::new::thread_start::h4a439a3f4cae6a0e
  50:     0x7fd5543f0609 - start_thread
  51:     0x7fd559d03133 - clone
  52:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (a960aa9c6 2022-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [opt_const_param_of] computing the optional const parameter of `main::{constant#0}`
#2 [typeck] type-checking `main::{constant#0}`
#3 [typeck_item_bodies] type-checking all item bodies
#4 [analysis] running analysis passes on this crate
error: aborting due to previous error

For more information about this error, try `rustc --explain E0107`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/const-generics/issues/issue-86530.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-86530.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-86530" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-86530/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'internal error: entered unreachable code', compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:107:46
stack backtrace:
   0:     0x7ff86e0f497c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5f278ce3b9a63834
   1:     0x7ff86e15afd8 - core::fmt::write::hfaaf3f7e811d7d79
   2:     0x7ff86e0e4761 - std::io::Write::write_fmt::h695784041433cf41
   3:     0x7ff86e0f79ce - std::panicking::default_hook::{{closure}}::h24ec1953f381a8f4
   4:     0x7ff86e0f75fc - std::panicking::default_hook::hb6535462245f12ca
   5:     0x7ff86ec8df11 - rustc_driver[eccc9f23e5d4e674]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7ff86e0f822e - std::panicking::rust_panic_with_hook::h67a5fa7ec0514204
   7:     0x7ff86e0f7fe9 - std::panicking::begin_panic_handler::{{closure}}::h4d41dac866db9bba
   8:     0x7ff86e0f4e94 - std::sys_common::backtrace::__rust_end_short_backtrace::h1a7b7284f6dba6e7
   9:     0x7ff86e0f7d09 - rust_begin_unwind
  10:     0x7ff86e0ac073 - core::panicking::panic_fmt::hca9d2d765596d598
  11:     0x7ff86e0abf3d - core::panicking::panic::h8a02272c1a26df46
  12:     0x7ff8712cfa0d - rustc_trait_selection[9563980304af3adb]::traits::const_evaluatable::is_const_evaluatable
  13:     0x7ff871218d21 - <rustc_trait_selection[9563980304af3adb]::traits::fulfill::FulfillProcessor>::process_changed_obligations
  14:     0x7ff871279c7e - <rustc_data_structures[67476deb84584d5d]::obligation_forest::ObligationForest<rustc_trait_selection[9563980304af3adb]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[9563980304af3adb]::traits::fulfill::FulfillProcessor, rustc_data_structures[67476deb84584d5d]::obligation_forest::Outcome<rustc_trait_selection[9563980304af3adb]::traits::fulfill::PendingPredicateObligation, rustc_infer[79f42d7d4b1ef223]::traits::FulfillmentErrorCode>>
  15:     0x7ff87121761b - <rustc_trait_selection[9563980304af3adb]::traits::fulfill::FulfillmentContext as rustc_infer[79f42d7d4b1ef223]::traits::engine::TraitEngine>::select_where_possible
  16:     0x7ff86f47a427 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_argument_types
  17:     0x7ff86f455a75 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::confirm_builtin_call
  18:     0x7ff86f453afb - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_call
  19:     0x7ff86f4be96e - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  20:     0x7ff86f4687a7 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  21:     0x7ff86f4bd799 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  22:     0x7ff86f48230b - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_stmt
  23:     0x7ff86f4828f4 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  24:     0x7ff86f4bec1a - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  25:     0x7ff86f4687a7 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  26:     0x7ff86f4bd799 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  27:     0x7ff86f469ba7 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_return_expr
  28:     0x7ff86f7cac82 - rustc_typeck[4a3bf33c0b57c62a]::check::check::check_fn
  29:     0x7ff86f67312e - <rustc_infer[79f42d7d4b1ef223]::infer::InferCtxtBuilder>::enter::<&rustc_middle[bbd1842e24523309]::ty::context::TypeckResults, <rustc_typeck[4a3bf33c0b57c62a]::check::inherited::InheritedBuilder>::enter<rustc_typeck[4a3bf33c0b57c62a]::check::typeck_with_fallback<rustc_typeck[4a3bf33c0b57c62a]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[bbd1842e24523309]::ty::context::TypeckResults>::{closure#0}>
  30:     0x7ff86f790e3e - <rustc_typeck[4a3bf33c0b57c62a]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[4a3bf33c0b57c62a]::check::typeck_with_fallback<rustc_typeck[4a3bf33c0b57c62a]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[bbd1842e24523309]::ty::context::TypeckResults>
  31:     0x7ff86f598afd - rustc_typeck[4a3bf33c0b57c62a]::check::typeck
  32:     0x7ff8706ce8e6 - rustc_query_system[5113d42b3843a911]::query::plumbing::try_execute_query::<rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt, rustc_query_system[5113d42b3843a911]::query::caches::DefaultCache<rustc_span[95f14e4ccbe972d3]::def_id::LocalDefId, &rustc_middle[bbd1842e24523309]::ty::context::TypeckResults>>
  33:     0x7ff8707e9f27 - rustc_query_system[5113d42b3843a911]::query::plumbing::get_query::<rustc_query_impl[25ec45a1d1991b1c]::queries::typeck, rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt>
  34:     0x7ff8703624c4 - <rustc_query_impl[25ec45a1d1991b1c]::Queries as rustc_middle[bbd1842e24523309]::ty::query::QueryEngine>::typeck
  35:     0x7ff86f735c5b - <rustc_middle[bbd1842e24523309]::hir::map::Map>::par_body_owners::<rustc_typeck[4a3bf33c0b57c62a]::check::typeck_item_bodies::{closure#0}>
  36:     0x7ff86f59d7ed - rustc_typeck[4a3bf33c0b57c62a]::check::typeck_item_bodies
  37:     0x7ff87071637a - rustc_query_system[5113d42b3843a911]::query::plumbing::try_execute_query::<rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt, rustc_query_system[5113d42b3843a911]::query::caches::DefaultCache<(), ()>>
  38:     0x7ff8707ada55 - rustc_query_system[5113d42b3843a911]::query::plumbing::get_query::<rustc_query_impl[25ec45a1d1991b1c]::queries::typeck_item_bodies, rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt>
  39:     0x7ff870361f6e - <rustc_query_impl[25ec45a1d1991b1c]::Queries as rustc_middle[bbd1842e24523309]::ty::query::QueryEngine>::typeck_item_bodies
  40:     0x7ff86f5fd7fa - <rustc_session[2266dbf95b3f99af]::session::Session>::time::<(), rustc_typeck[4a3bf33c0b57c62a]::check_crate::{closure#7}>
  41:     0x7ff86f5f9db8 - rustc_typeck[4a3bf33c0b57c62a]::check_crate
  42:     0x7ff86ed62691 - rustc_interface[2c90a0c0bfdfedc]::passes::analysis
  43:     0x7ff87070aa3e - rustc_query_system[5113d42b3843a911]::query::plumbing::try_execute_query::<rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt, rustc_query_system[5113d42b3843a911]::query::caches::DefaultCache<(), core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>>
  44:     0x7ff8707ea302 - rustc_query_system[5113d42b3843a911]::query::plumbing::get_query::<rustc_query_impl[25ec45a1d1991b1c]::queries::analysis, rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt>
  45:     0x7ff870345e7e - <rustc_query_impl[25ec45a1d1991b1c]::Queries as rustc_middle[bbd1842e24523309]::ty::query::QueryEngine>::analysis
  46:     0x7ff86ec702f4 - <rustc_interface[2c90a0c0bfdfedc]::passes::QueryContext>::enter::<rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>
  47:     0x7ff86ec1c60e - <rustc_interface[2c90a0c0bfdfedc]::interface::Compiler>::enter::<rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}::{closure#2}, core[5e9b674adaad8ecd]::result::Result<core[5e9b674adaad8ecd]::option::Option<rustc_interface[2c90a0c0bfdfedc]::queries::Linker>, rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>
  48:     0x7ff86ebfe30b - rustc_span[95f14e4ccbe972d3]::with_source_map::<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_interface[2c90a0c0bfdfedc]::interface::create_compiler_and_run<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}>::{closure#1}>
  49:     0x7ff86ec1d7a9 - <scoped_tls[5fdc27eea9245943]::ScopedKey<rustc_span[95f14e4ccbe972d3]::SessionGlobals>>::set::<rustc_interface[2c90a0c0bfdfedc]::interface::run_compiler<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>
  50:     0x7ff86ec79239 - std[cf1d69d5cbb98166]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2c90a0c0bfdfedc]::util::run_in_thread_pool_with_globals<rustc_interface[2c90a0c0bfdfedc]::interface::run_compiler<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>
  51:     0x7ff86ec31f01 - std[cf1d69d5cbb98166]::panicking::try::<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, core[5e9b674adaad8ecd]::panic::unwind_safe::AssertUnwindSafe<<std[cf1d69d5cbb98166]::thread::Builder>::spawn_unchecked_<rustc_interface[2c90a0c0bfdfedc]::util::run_in_thread_pool_with_globals<rustc_interface[2c90a0c0bfdfedc]::interface::run_compiler<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  52:     0x7ff86ec73ea2 - <<std[cf1d69d5cbb98166]::thread::Builder>::spawn_unchecked_<rustc_interface[2c90a0c0bfdfedc]::util::run_in_thread_pool_with_globals<rustc_interface[2c90a0c0bfdfedc]::interface::run_compiler<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>::{closure#1} as core[5e9b674adaad8ecd]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  53:     0x7ff86e103603 - std::sys::unix::thread::Thread::new::thread_start::h4a439a3f4cae6a0e
  54:     0x7ff868655609 - start_thread
  55:     0x7ff86df68133 - clone
  56:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (a960aa9c6 2022-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `unit_literals`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] src/test/ui/const-generics/occurs-check/unused-substs-3.rs stdout ----
---- [ui] src/test/ui/const-generics/occurs-check/unused-substs-3.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/occurs-check/unused-substs-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/occurs-check/unused-substs-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/occurs-check/unused-substs-3/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'internal error: entered unreachable code', compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:107:46
   0:     0x7f2e0a1d397c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5f278ce3b9a63834
   1:     0x7f2e0a239fd8 - core::fmt::write::hfaaf3f7e811d7d79
   2:     0x7f2e0a1c3761 - std::io::Write::write_fmt::h695784041433cf41
   2:     0x7f2e0a1c3761 - std::io::Write::write_fmt::h695784041433cf41
   3:     0x7f2e0a1d69ce - std::panicking::default_hook::{{closure}}::h24ec1953f381a8f4
   4:     0x7f2e0a1d65fc - std::panicking::default_hook::hb6535462245f12ca
   5:     0x7f2e0ad6cf11 - rustc_driver[eccc9f23e5d4e674]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f2e0a1d722e - std::panicking::rust_panic_with_hook::h67a5fa7ec0514204
   7:     0x7f2e0a1d6fe9 - std::panicking::begin_panic_handler::{{closure}}::h4d41dac866db9bba
   8:     0x7f2e0a1d3e94 - std::sys_common::backtrace::__rust_end_short_backtrace::h1a7b7284f6dba6e7
   9:     0x7f2e0a1d6d09 - rust_begin_unwind
  10:     0x7f2e0a18b073 - core::panicking::panic_fmt::hca9d2d765596d598
  11:     0x7f2e0a18af3d - core::panicking::panic::h8a02272c1a26df46
  12:     0x7f2e0d3aea0d - rustc_trait_selection[9563980304af3adb]::traits::const_evaluatable::is_const_evaluatable
  13:     0x7f2e0d2f7d21 - <rustc_trait_selection[9563980304af3adb]::traits::fulfill::FulfillProcessor>::process_changed_obligations
  14:     0x7f2e0d358c7e - <rustc_data_structures[67476deb84584d5d]::obligation_forest::ObligationForest<rustc_trait_selection[9563980304af3adb]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[9563980304af3adb]::traits::fulfill::FulfillProcessor, rustc_data_structures[67476deb84584d5d]::obligation_forest::Outcome<rustc_trait_selection[9563980304af3adb]::traits::fulfill::PendingPredicateObligation, rustc_infer[79f42d7d4b1ef223]::traits::FulfillmentErrorCode>>
  15:     0x7f2e0d2f661b - <rustc_trait_selection[9563980304af3adb]::traits::fulfill::FulfillmentContext as rustc_infer[79f42d7d4b1ef223]::traits::engine::TraitEngine>::select_where_possible
  16:     0x7f2e0b552cad - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::resolve_vars_with_obligations
  17:     0x7f2e0b55814a - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::structurally_resolved_type
  18:     0x7f2e0b532000 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_call
  19:     0x7f2e0b59d96e - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  20:     0x7f2e0b5477a7 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  21:     0x7f2e0b59c799 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  22:     0x7f2e0b560fa5 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_decl
  23:     0x7f2e0b5611b3 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_stmt
  24:     0x7f2e0b5618f4 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  25:     0x7f2e0b59dc1a - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  26:     0x7f2e0b5477a7 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  27:     0x7f2e0b59c799 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  28:     0x7f2e0b548ba7 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_return_expr
  29:     0x7f2e0b8a9c82 - rustc_typeck[4a3bf33c0b57c62a]::check::check::check_fn
  30:     0x7f2e0b75212e - <rustc_infer[79f42d7d4b1ef223]::infer::InferCtxtBuilder>::enter::<&rustc_middle[bbd1842e24523309]::ty::context::TypeckResults, <rustc_typeck[4a3bf33c0b57c62a]::check::inherited::InheritedBuilder>::enter<rustc_typeck[4a3bf33c0b57c62a]::check::typeck_with_fallback<rustc_typeck[4a3bf33c0b57c62a]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[bbd1842e24523309]::ty::context::TypeckResults>::{closure#0}>
  31:     0x7f2e0b86fe3e - <rustc_typeck[4a3bf33c0b57c62a]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[4a3bf33c0b57c62a]::check::typeck_with_fallback<rustc_typeck[4a3bf33c0b57c62a]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[bbd1842e24523309]::ty::context::TypeckResults>
  32:     0x7f2e0b677afd - rustc_typeck[4a3bf33c0b57c62a]::check::typeck
  33:     0x7f2e0c7ad8e6 - rustc_query_system[5113d42b3843a911]::query::plumbing::try_execute_query::<rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt, rustc_query_system[5113d42b3843a911]::query::caches::DefaultCache<rustc_span[95f14e4ccbe972d3]::def_id::LocalDefId, &rustc_middle[bbd1842e24523309]::ty::context::TypeckResults>>
  34:     0x7f2e0c8c8f27 - rustc_query_system[5113d42b3843a911]::query::plumbing::get_query::<rustc_query_impl[25ec45a1d1991b1c]::queries::typeck, rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt>
  35:     0x7f2e0c4414c4 - <rustc_query_impl[25ec45a1d1991b1c]::Queries as rustc_middle[bbd1842e24523309]::ty::query::QueryEngine>::typeck
  36:     0x7f2e0b814c5b - <rustc_middle[bbd1842e24523309]::hir::map::Map>::par_body_owners::<rustc_typeck[4a3bf33c0b57c62a]::check::typeck_item_bodies::{closure#0}>
  37:     0x7f2e0b67c7ed - rustc_typeck[4a3bf33c0b57c62a]::check::typeck_item_bodies
  38:     0x7f2e0c7f537a - rustc_query_system[5113d42b3843a911]::query::plumbing::try_execute_query::<rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt, rustc_query_system[5113d42b3843a911]::query::caches::DefaultCache<(), ()>>
  39:     0x7f2e0c88ca55 - rustc_query_system[5113d42b3843a911]::query::plumbing::get_query::<rustc_query_impl[25ec45a1d1991b1c]::queries::typeck_item_bodies, rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt>
  40:     0x7f2e0c440f6e - <rustc_query_impl[25ec45a1d1991b1c]::Queries as rustc_middle[bbd1842e24523309]::ty::query::QueryEngine>::typeck_item_bodies
  41:     0x7f2e0b6dc7fa - <rustc_session[2266dbf95b3f99af]::session::Session>::time::<(), rustc_typeck[4a3bf33c0b57c62a]::check_crate::{closure#7}>
  42:     0x7f2e0b6d8db8 - rustc_typeck[4a3bf33c0b57c62a]::check_crate
  43:     0x7f2e0ae41691 - rustc_interface[2c90a0c0bfdfedc]::passes::analysis
  44:     0x7f2e0c7e9a3e - rustc_query_system[5113d42b3843a911]::query::plumbing::try_execute_query::<rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt, rustc_query_system[5113d42b3843a911]::query::caches::DefaultCache<(), core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>>
  45:     0x7f2e0c8c9302 - rustc_query_system[5113d42b3843a911]::query::plumbing::get_query::<rustc_query_impl[25ec45a1d1991b1c]::queries::analysis, rustc_query_impl[25ec45a1d1991b1c]::plumbing::QueryCtxt>
  46:     0x7f2e0c424e7e - <rustc_query_impl[25ec45a1d1991b1c]::Queries as rustc_middle[bbd1842e24523309]::ty::query::QueryEngine>::analysis
  47:     0x7f2e0ad4f2f4 - <rustc_interface[2c90a0c0bfdfedc]::passes::QueryContext>::enter::<rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>
  48:     0x7f2e0acfb60e - <rustc_interface[2c90a0c0bfdfedc]::interface::Compiler>::enter::<rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}::{closure#2}, core[5e9b674adaad8ecd]::result::Result<core[5e9b674adaad8ecd]::option::Option<rustc_interface[2c90a0c0bfdfedc]::queries::Linker>, rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>
  49:     0x7f2e0acdd30b - rustc_span[95f14e4ccbe972d3]::with_source_map::<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_interface[2c90a0c0bfdfedc]::interface::create_compiler_and_run<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}>::{closure#1}>
  50:     0x7f2e0acfc7a9 - <scoped_tls[5fdc27eea9245943]::ScopedKey<rustc_span[95f14e4ccbe972d3]::SessionGlobals>>::set::<rustc_interface[2c90a0c0bfdfedc]::interface::run_compiler<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>
  51:     0x7f2e0ad58239 - std[cf1d69d5cbb98166]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2c90a0c0bfdfedc]::util::run_in_thread_pool_with_globals<rustc_interface[2c90a0c0bfdfedc]::interface::run_compiler<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>
  52:     0x7f2e0ad10f01 - std[cf1d69d5cbb98166]::panicking::try::<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, core[5e9b674adaad8ecd]::panic::unwind_safe::AssertUnwindSafe<<std[cf1d69d5cbb98166]::thread::Builder>::spawn_unchecked_<rustc_interface[2c90a0c0bfdfedc]::util::run_in_thread_pool_with_globals<rustc_interface[2c90a0c0bfdfedc]::interface::run_compiler<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  53:     0x7f2e0ad52ea2 - <<std[cf1d69d5cbb98166]::thread::Builder>::spawn_unchecked_<rustc_interface[2c90a0c0bfdfedc]::util::run_in_thread_pool_with_globals<rustc_interface[2c90a0c0bfdfedc]::interface::run_compiler<core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>, rustc_driver[eccc9f23e5d4e674]::run_compiler::{closure#1}>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>::{closure#0}, core[5e9b674adaad8ecd]::result::Result<(), rustc_errors[8c6bdbf4557aa9db]::ErrorGuaranteed>>::{closure#1} as core[5e9b674adaad8ecd]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  54:     0x7f2e0a1e2603 - std::sys::unix::thread::Thread::new::thread_start::h4a439a3f4cae6a0e
  55:     0x7f2e04734609 - start_thread
  56:     0x7f2e0a047133 - clone
  57:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (a960aa9c6 2022-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] src/test/ui/const-generics/occurs-check/unused-substs-4.rs stdout ----
---- [ui] src/test/ui/const-generics/occurs-check/unused-substs-4.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/occurs-check/unused-substs-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/occurs-check/unused-substs-4" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/occurs-check/unused-substs-4/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'internal error: entered unreachable code', compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:107:46
stack backtrace:
   0:     0x7f4a21e8d97c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5f278ce3b9a63834
   1:     0x7f4a21ef3fd8 - core::fmt::write::hfaaf3f7e811d7d79
   2:     0x7f4a21e7d761 - std::io::Write::write_fmt::h695784041433cf41
   3:     0x7f4a21e909ce - std::panicking::default_hook::{{closure}}::h24ec1953f381a8f4
   4:     0x7f4a21e905fc - std::panicking::default_hook::hb6535462245f12ca
   5:     0x7f4a22a26f11 - rustc_driver[eccc9f23e5d4e674]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f4a21e9122e - std::panicking::rust_panic_with_hook::h67a5fa7ec0514204
   7:     0x7f4a21e90fe9 - std::panicking::begin_panic_handler::{{closure}}::h4d41dac866db9bba
   8:     0x7f4a21e8de94 - std::sys_common::backtrace::__rust_end_short_backtrace::h1a7b7284f6dba6e7
   9:     0x7f4a21e90d09 - rust_begin_unwind
  10:     0x7f4a21e45073 - core::panicking::panic_fmt::hca9d2d765596d598
  11:     0x7f4a21e44f3d - core::panicking::panic::h8a02272c1a26df46
  12:     0x7f4a25068a0d - rustc_trait_selection[9563980304af3adb]::traits::const_evaluatable::is_const_evaluatable
  13:     0x7f4a24fb1d21 - <rustc_trait_selection[9563980304af3adb]::traits::fulfill::FulfillProcessor>::process_changed_obligations
  14:     0x7f4a25012c7e - <rustc_data_structures[67476deb84584d5d]::obligation_forest::ObligationForest<rustc_trait_selection[9563980304af3adb]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[9563980304af3adb]::traits::fulfill::FulfillProcessor, rustc_data_structures[67476deb84584d5d]::obligation_forest::Outcome<rustc_trait_selection[9563980304af3adb]::traits::fulfill::PendingPredicateObligation, rustc_infer[79f42d7d4b1ef223]::traits::FulfillmentErrorCode>>
  15:     0x7f4a24fb061b - <rustc_trait_selection[9563980304af3adb]::traits::fulfill::FulfillmentContext as rustc_infer[79f42d7d4b1ef223]::traits::engine::TraitEngine>::select_where_possible
  16:     0x7f4a2320ccad - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::resolve_vars_with_obligations
  17:     0x7f4a2321214a - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::structurally_resolved_type
  18:     0x7f4a231ec000 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_call
  19:     0x7f4a2325796e - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  20:     0x7f4a232017a7 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  21:     0x7f4a23256799 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  22:     0x7f4a2325ea20 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  23:     0x7f4a232017a7 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  24:     0x7f4a23256799 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  25:     0x7f4a2321b30b - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_stmt
  26:     0x7f4a2321b8f4 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  27:     0x7f4a23257c1a - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  28:     0x7f4a232017a7 - <rustc_typeck[4a3bf33c0b57c62a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
