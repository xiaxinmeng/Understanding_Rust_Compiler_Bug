plain
failures:

---- [ui] src/test/ui/const-generics/issues/issue-86530.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-86530.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-86530" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-86530/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'unexpected `TooGeneric` for Unevaluated { def: WithOptConstParam { did: DefId(0:7 ~ issue_86530[124d]::z::{constant#0}), const_param_did: None }, substs: [&str], promoted: () }', compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:101:17
stack backtrace:
   0:     0x7f3dd583b87c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha0675371b7e351f7
   1:     0x7f3dd58a1d58 - core::fmt::write::h6087bd2adf09fd27
   2:     0x7f3dd582b681 - std::io::Write::write_fmt::h89935baa8cc17bbc
   3:     0x7f3dd583e88e - std::panicking::default_hook::{{closure}}::h46ab91d7e076c500
   4:     0x7f3dd583e579 - std::panicking::default_hook::h3d5baa541099c8b4
   5:     0x7f3dd63d85a1 - rustc_driver[1a1790f2e9c95b09]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f3dd583eff2 - std::panicking::rust_panic_with_hook::hc366189e932e2653
   7:     0x7f3dd583ee27 - std::panicking::begin_panic_handler::{{closure}}::hd67379ad9acd32c9
   8:     0x7f3dd583bd94 - std::sys_common::backtrace::__rust_end_short_backtrace::h6492d4f10a86f86d
   9:     0x7f3dd583eb09 - rust_begin_unwind
  10:     0x7f3dd57f3013 - core::panicking::panic_fmt::he2a2411f3f6baf17
  11:     0x7f3dd8a2aac7 - rustc_trait_selection[de70ba4d3c32bab6]::traits::const_evaluatable::is_const_evaluatable
  12:     0x7f3dd8973533 - <rustc_trait_selection[de70ba4d3c32bab6]::traits::fulfill::FulfillProcessor>::process_changed_obligations
  13:     0x7f3dd89d4a4e - <rustc_data_structures[4f0b228267343faa]::obligation_forest::ObligationForest<rustc_trait_selection[de70ba4d3c32bab6]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[de70ba4d3c32bab6]::traits::fulfill::FulfillProcessor, rustc_data_structures[4f0b228267343faa]::obligation_forest::Outcome<rustc_trait_selection[de70ba4d3c32bab6]::traits::fulfill::PendingPredicateObligation, rustc_infer[30cff0c6e72f9abd]::traits::FulfillmentErrorCode>>
  14:     0x7f3dd8971e0b - <rustc_trait_selection[de70ba4d3c32bab6]::traits::fulfill::FulfillmentContext as rustc_infer[30cff0c6e72f9abd]::traits::engine::TraitEngine>::select_where_possible
  15:     0x7f3dd6bc33f7 - <rustc_typeck[6c5d31e594aaa7c5]::check::fn_ctxt::FnCtxt>::check_argument_types
  16:     0x7f3dd6b9ed8c - <rustc_typeck[6c5d31e594aaa7c5]::check::fn_ctxt::FnCtxt>::confirm_builtin_call
  17:     0x7f3dd6b9ce0b - <rustc_typeck[6c5d31e594aaa7c5]::check::fn_ctxt::FnCtxt>::check_call
  18:     0x7f3dd6c08c3e - <rustc_typeck[6c5d31e594aaa7c5]::check::fn_ctxt::FnCtxt>::check_expr_kind
  19:     0x7f3dd6bb15d7 - <rustc_typeck[6c5d31e594aaa7c5]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  20:     0x7f3dd6c07a69 - <rustc_typeck[6c5d31e594aaa7c5]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  21:     0x7f3dd6bcb2f0 - <rustc_typeck[6c5d31e594aaa7c5]::check::fn_ctxt::FnCtxt>::check_stmt
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  22:     0x7f3dd6bcb8e4 - <rustc_typeck[6c5d31e594aaa7c5]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  23:     0x7f3dd6c08eea - <rustc_typeck[6c5d31e594aaa7c5]::check::fn_ctxt::FnCtxt>::check_expr_kind
  24:     0x7f3dd6bb15d7 - <rustc_typeck[6c5d31e594aaa7c5]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  25:     0x7f3dd6c07a69 - <rustc_typeck[6c5d31e594aaa7c5]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  26:     0x7f3dd6bb29d7 - <rustc_typeck[6c5d31e594aaa7c5]::check::fn_ctxt::FnCtxt>::check_return_expr
  27:     0x7f3dd6c99ba2 - rustc_typeck[6c5d31e594aaa7c5]::check::check::check_fn
  28:     0x7f3dd6dcaa6e - <rustc_infer[30cff0c6e72f9abd]::infer::InferCtxtBuilder>::enter::<&rustc_middle[cbeb590da263f883]::ty::context::TypeckResults, <rustc_typeck[6c5d31e594aaa7c5]::check::inherited::InheritedBuilder>::enter<rustc_typeck[6c5d31e594aaa7c5]::check::typeck_with_fallback<rustc_typeck[6c5d31e594aaa7c5]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[cbeb590da263f883]::ty::context::TypeckResults>::{closure#0}>
  29:     0x7f3dd6f11dee - <rustc_typeck[6c5d31e594aaa7c5]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[6c5d31e594aaa7c5]::check::typeck_with_fallback<rustc_typeck[6c5d31e594aaa7c5]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[cbeb590da263f883]::ty::context::TypeckResults>
  30:     0x7f3dd6ce9a31 - rustc_typeck[6c5d31e594aaa7c5]::check::typeck
  31:     0x7f3dd7e1a8e6 - rustc_query_system[15c242cc594b17ee]::query::plumbing::try_execute_query::<rustc_query_impl[1b263358b3e13738]::plumbing::QueryCtxt, rustc_query_system[15c242cc594b17ee]::query::caches::DefaultCache<rustc_span[525427bcc8998ac3]::def_id::LocalDefId, &rustc_middle[cbeb590da263f883]::ty::context::TypeckResults>>
  32:     0x7f3dd7f345b7 - rustc_query_system[15c242cc594b17ee]::query::plumbing::get_query::<rustc_query_impl[1b263358b3e13738]::queries::typeck, rustc_query_impl[1b263358b3e13738]::plumbing::QueryCtxt>
  33:     0x7f3dd7a93644 - <rustc_query_impl[1b263358b3e13738]::Queries as rustc_middle[cbeb590da263f883]::ty::query::QueryEngine>::typeck
  34:     0x7f3dd6ec123a - <rustc_middle[cbeb590da263f883]::hir::map::Map>::par_body_owners::<rustc_typeck[6c5d31e594aaa7c5]::check::typeck_item_bodies::{closure#0}>
  35:     0x7f3dd6cee81d - rustc_typeck[6c5d31e594aaa7c5]::check::typeck_item_bodies
  36:     0x7f3dd7e6151a - rustc_query_system[15c242cc594b17ee]::query::plumbing::try_execute_query::<rustc_query_impl[1b263358b3e13738]::plumbing::QueryCtxt, rustc_query_system[15c242cc594b17ee]::query::caches::DefaultCache<(), ()>>
  37:     0x7f3dd7ef80e5 - rustc_query_system[15c242cc594b17ee]::query::plumbing::get_query::<rustc_query_impl[1b263358b3e13738]::queries::typeck_item_bodies, rustc_query_impl[1b263358b3e13738]::plumbing::QueryCtxt>
  38:     0x7f3dd7a930ee - <rustc_query_impl[1b263358b3e13738]::Queries as rustc_middle[cbeb590da263f883]::ty::query::QueryEngine>::typeck_item_bodies
  39:     0x7f3dd6dbb4ba - <rustc_session[e6936a89584ab06f]::session::Session>::time::<(), rustc_typeck[6c5d31e594aaa7c5]::check_crate::{closure#7}>
  40:     0x7f3dd6d7fa66 - rustc_typeck[6c5d31e594aaa7c5]::check_crate
  41:     0x7f3dd64ac9f1 - rustc_interface[62b197190d2e67c0]::passes::analysis
  42:     0x7f3dd7e55bde - rustc_query_system[15c242cc594b17ee]::query::plumbing::try_execute_query::<rustc_query_impl[1b263358b3e13738]::plumbing::QueryCtxt, rustc_query_system[15c242cc594b17ee]::query::caches::DefaultCache<(), core[f93763e804187272]::result::Result<(), rustc_errors[f2b691f8b3659dab]::ErrorGuaranteed>>>
  43:     0x7f3dd7f34992 - rustc_query_system[15c242cc594b17ee]::query::plumbing::get_query::<rustc_query_impl[1b263358b3e13738]::queries::analysis, rustc_query_impl[1b263358b3e13738]::plumbing::QueryCtxt>
  44:     0x7f3dd7a76ffe - <rustc_query_impl[1b263358b3e13738]::Queries as rustc_middle[cbeb590da263f883]::ty::query::QueryEngine>::analysis
  45:     0x7f3dd63bb624 - <rustc_interface[62b197190d2e67c0]::passes::QueryContext>::enter::<rustc_driver[1a1790f2e9c95b09]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[f93763e804187272]::result::Result<(), rustc_errors[f2b691f8b3659dab]::ErrorGuaranteed>>
  46:     0x7f3dd636868e - <rustc_interface[62b197190d2e67c0]::interface::Compiler>::enter::<rustc_driver[1a1790f2e9c95b09]::run_compiler::{closure#1}::{closure#2}, core[f93763e804187272]::result::Result<core[f93763e804187272]::option::Option<rustc_interface[62b197190d2e67c0]::queries::Linker>, rustc_errors[f2b691f8b3659dab]::ErrorGuaranteed>>
  47:     0x7f3dd634a4eb - rustc_span[525427bcc8998ac3]::with_source_map::<core[f93763e804187272]::result::Result<(), rustc_errors[f2b691f8b3659dab]::ErrorGuaranteed>, rustc_interface[62b197190d2e67c0]::interface::create_compiler_and_run<core[f93763e804187272]::result::Result<(), rustc_errors[f2b691f8b3659dab]::ErrorGuaranteed>, rustc_driver[1a1790f2e9c95b09]::run_compiler::{closure#1}>::{closure#1}>
  48:     0x7f3dd6369829 - <scoped_tls[74679fca20b8b099]::ScopedKey<rustc_span[525427bcc8998ac3]::SessionGlobals>>::set::<rustc_interface[62b197190d2e67c0]::interface::run_compiler<core[f93763e804187272]::result::Result<(), rustc_errors[f2b691f8b3659dab]::ErrorGuaranteed>, rustc_driver[1a1790f2e9c95b09]::run_compiler::{closure#1}>::{closure#0}, core[f93763e804187272]::result::Result<(), rustc_errors[f2b691f8b3659dab]::ErrorGuaranteed>>
  49:     0x7f3dd63bf269 - std[db0f246162eb22f4]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[62b197190d2e67c0]::util::run_in_thread_pool_with_globals<rustc_interface[62b197190d2e67c0]::interface::run_compiler<core[f93763e804187272]::result::Result<(), rustc_errors[f2b691f8b3659dab]::ErrorGuaranteed>, rustc_driver[1a1790f2e9c95b09]::run_compiler::{closure#1}>::{closure#0}, core[f93763e804187272]::result::Result<(), rustc_errors[f2b691f8b3659dab]::ErrorGuaranteed>>::{closure#0}, core[f93763e804187272]::result::Result<(), rustc_errors[f2b691f8b3659dab]::ErrorGuaranteed>>
  50:     0x7f3dd636af91 - std[db0f246162eb22f4]::panicking::try::<core[f93763e804187272]::result::Result<(), rustc_errors[f2b691f8b3659dab]::ErrorGuaranteed>, core[f93763e804187272]::panic::unwind_safe::AssertUnwindSafe<<std[db0f246162eb22f4]::thread::Builder>::spawn_unchecked_<rustc_interface[62b197190d2e67c0]::util::run_in_thread_pool_with_globals<rustc_interface[62b197190d2e67c0]::interface::run_compiler<core[f93763e804187272]::result::Result<(), rustc_errors[f2b691f8b3659dab]::ErrorGuaranteed>, rustc_driver[1a1790f2e9c95b09]::run_compiler::{closure#1}>::{closure#0}, core[f93763e804187272]::result::Result<(), rustc_errors[f2b691f8b3659dab]::ErrorGuaranteed>>::{closure#0}, core[f93763e804187272]::result::Result<(), rustc_errors[f2b691f8b3659dab]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  51:     0x7f3dd63bff82 - <<std[db0f246162eb22f4]::thread::Builder>::spawn_unchecked_<rustc_interface[62b197190d2e67c0]::util::run_in_thread_pool_with_globals<rustc_interface[62b197190d2e67c0]::interface::run_compiler<core[f93763e804187272]::result::Result<(), rustc_errors[f2b691f8b3659dab]::ErrorGuaranteed>, rustc_driver[1a1790f2e9c95b09]::run_compiler::{closure#1}>::{closure#0}, core[f93763e804187272]::result::Result<(), rustc_errors[f2b691f8b3659dab]::ErrorGuaranteed>>::{closure#0}, core[f93763e804187272]::result::Result<(), rustc_errors[f2b691f8b3659dab]::ErrorGuaranteed>>::{closure#1} as core[f93763e804187272]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  52:     0x7f3dd584a383 - std::sys::unix::thread::Thread::new::thread_start::h32fbc88fe74591d1
  53:     0x7f3dcfd9c609 - start_thread
  54:     0x7f3dd56af133 - clone
  55:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (7f902354e 2022-05-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `unit_literals`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
------------------------------------------



