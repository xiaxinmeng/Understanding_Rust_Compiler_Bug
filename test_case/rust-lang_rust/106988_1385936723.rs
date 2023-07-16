plain

running 14197 tests
..........................................ii............................................ 88/14197
.................................................................................iiiiiii 176/14197
iiiiiiii.....................i..................i.............................F......... 264/14197
...........................F.............................................F...F.......... 352/14197
........................................................................................ 528/14197
........................................................................................ 616/14197
........................................................................................ 704/14197
........................................................................................ 792/14197
---
---- [ui] checkout/tests/ui/associated-consts/assoc-const.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/associated-consts/assoc-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/assoc-const/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/assoc-const/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_middle/src/ty/sty.rs:1264:21: unexpected DefKind in ProjectionTy: AssocConst
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1609:9
stack backtrace:
   0:     0x7fa5566de825 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h38611d6c58ff15d7
   1:     0x7fa55674ed18 - core::fmt::write::h2d75faa94db0a514
   1:     0x7fa55674ed18 - core::fmt::write::h2d75faa94db0a514
   2:     0x7fa5566d06d1 - std::io::Write::write_fmt::h00bd98e74899b603
   3:     0x7fa5566de631 - std::sys_common::backtrace::print::h805af3402a33b9e0
   4:     0x7fa5566e1a14 - std::panicking::default_hook::{{closure}}::hd3489b51e502c13f
   5:     0x7fa5566e16da - std::panicking::default_hook::hcb306b30123f73b7
   6:     0x7fa557150b82 - rustc_driver[24f6a6ae78ae4804]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fa5566e2184 - std::panicking::rust_panic_with_hook::h87fc5763cb93e40f
   8:     0x7fa559e56f53 - std[4fc4c1d6570de2f7]::panicking::begin_panic::<rustc_errors[693af92fc8bcbc7a]::ExplicitBug>::{closure#0}
   9:     0x7fa559e56496 - std[4fc4c1d6570de2f7]::sys_common::backtrace::__rust_end_short_backtrace::<std[4fc4c1d6570de2f7]::panicking::begin_panic<rustc_errors[693af92fc8bcbc7a]::ExplicitBug>::{closure#0}, !>
  10:     0x7fa5570f7956 - std[4fc4c1d6570de2f7]::panicking::begin_panic::<rustc_errors[693af92fc8bcbc7a]::ExplicitBug>
  11:     0x7fa559fe6826 - std[4fc4c1d6570de2f7]::panic::panic_any::<rustc_errors[693af92fc8bcbc7a]::ExplicitBug>
  12:     0x7fa559fe5ce0 - <rustc_errors[693af92fc8bcbc7a]::HandlerInner>::bug::<&alloc[a656fae0f04844bf]::string::String>
  13:     0x7fa559fe58a0 - <rustc_errors[693af92fc8bcbc7a]::Handler>::bug::<&alloc[a656fae0f04844bf]::string::String>
  14:     0x7fa55a037775 - rustc_middle[86905b0aa7050cb2]::util::bug::opt_span_bug_fmt::<rustc_span[fff9630aea768480]::span_encoding::Span>::{closure#0}
  15:     0x7fa55a035ffc - rustc_middle[86905b0aa7050cb2]::ty::context::tls::with_opt::<rustc_middle[86905b0aa7050cb2]::util::bug::opt_span_bug_fmt<rustc_span[fff9630aea768480]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  16:     0x7fa55a035fa6 - rustc_middle[86905b0aa7050cb2]::ty::context::tls::with_context_opt::<rustc_middle[86905b0aa7050cb2]::ty::context::tls::with_opt<rustc_middle[86905b0aa7050cb2]::util::bug::opt_span_bug_fmt<rustc_span[fff9630aea768480]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  17:     0x7fa55a0376b9 - rustc_middle[86905b0aa7050cb2]::util::bug::opt_span_bug_fmt::<rustc_span[fff9630aea768480]::span_encoding::Span>
  18:     0x7fa5570f7905 - rustc_middle[86905b0aa7050cb2]::util::bug::bug_fmt
  19:     0x7fa559f978ed - <rustc_middle[86905b0aa7050cb2]::ty::sty::AliasTy>::trait_def_id
  20:     0x7fa559f97c1f - <rustc_middle[86905b0aa7050cb2]::ty::sty::AliasTy>::trait_ref
  21:     0x7fa559aa1e0a - rustc_trait_selection[ce79bbcd8c7b125d]::traits::project::opt_normalize_projection_type
  22:     0x7fa559a987b5 - rustc_trait_selection[ce79bbcd8c7b125d]::traits::project::project_and_unify_type
  23:     0x7fa559a6be70 - <rustc_infer[965322cfb50022e7]::infer::InferCtxt>::commit_if_ok::<rustc_trait_selection[ce79bbcd8c7b125d]::traits::project::ProjectAndUnifyResult, rustc_infer[965322cfb50022e7]::traits::project::MismatchedProjectionTypes, rustc_trait_selection[ce79bbcd8c7b125d]::traits::project::poly_project_and_unify_type::{closure#0}>
  24:     0x7fa559a98326 - rustc_trait_selection[ce79bbcd8c7b125d]::traits::project::poly_project_and_unify_type
  25:     0x7fa559bb093a - <rustc_trait_selection[ce79bbcd8c7b125d]::traits::select::SelectionContext>::evaluate_predicate_recursively
  26:     0x7fa559a72273 - <rustc_infer[965322cfb50022e7]::infer::InferCtxt>::probe::<core[79021a402131dad8]::result::Result<rustc_middle[86905b0aa7050cb2]::traits::select::EvaluationResult, rustc_middle[86905b0aa7050cb2]::traits::select::OverflowError>, <rustc_trait_selection[ce79bbcd8c7b125d]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[ce79bbcd8c7b125d]::traits::select::SelectionContext>::evaluate_root_obligation::{closure#0}>::{closure#0}>
  27:     0x7fa559b94b7d - <rustc_trait_selection[ce79bbcd8c7b125d]::traits::select::SelectionContext>::evaluate_root_obligation
  28:     0x7fa558801acf - rustc_traits[d705a9fda77a3464]::evaluate_obligation::evaluate_obligation
  29:     0x7fa558fca10d - rustc_query_system[4bcb50d269c32ba]::query::plumbing::get_query::<rustc_query_impl[68facdf58d22355b]::queries::evaluate_obligation, rustc_query_impl[68facdf58d22355b]::plumbing::QueryCtxt, rustc_middle[86905b0aa7050cb2]::dep_graph::dep_node::DepKind>
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  30:     0x7fa558c59538 - <rustc_query_impl[68facdf58d22355b]::Queries as rustc_middle[86905b0aa7050cb2]::ty::query::QueryEngine>::evaluate_obligation
  31:     0x7fa559a81f8a - <rustc_infer[965322cfb50022e7]::infer::InferCtxt as rustc_trait_selection[ce79bbcd8c7b125d]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation
  32:     0x7fa559a8212c - <rustc_infer[965322cfb50022e7]::infer::InferCtxt as rustc_trait_selection[ce79bbcd8c7b125d]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation_no_overflow
  33:     0x7fa559c0345f - <rustc_trait_selection[ce79bbcd8c7b125d]::traits::fulfill::FulfillProcessor>::process_projection_obligation
  34:     0x7fa559c0d80f - <rustc_trait_selection[ce79bbcd8c7b125d]::traits::fulfill::FulfillProcessor as rustc_data_structures[9f90b09327a9efb6]::obligation_forest::ObligationProcessor>::process_obligation
  35:     0x7fa559adf6a3 - <rustc_data_structures[9f90b09327a9efb6]::obligation_forest::ObligationForest<rustc_trait_selection[ce79bbcd8c7b125d]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[ce79bbcd8c7b125d]::traits::fulfill::FulfillProcessor>
  36:     0x7fa559c02ba7 - <rustc_trait_selection[ce79bbcd8c7b125d]::traits::fulfill::FulfillmentContext as rustc_infer[965322cfb50022e7]::traits::engine::TraitEngine>::select_where_possible
  37:     0x7fa557699dd5 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_argument_types
  38:     0x7fa557668991 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::confirm_builtin_call
  39:     0x7fa55766771a - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_call
  40:     0x7fa5576eba92 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_expr_kind
  41:     0x7fa557681a71 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  42:     0x7fa5576ea8d2 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  43:     0x7fa5576a20a6 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_block_with_expected
  44:     0x7fa5576ebdd4 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_expr_kind
  45:     0x7fa557681a71 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  46:     0x7fa5576ea8d2 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  47:     0x7fa557683b9b - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_return_expr
  48:     0x7fa5577d8997 - rustc_hir_typeck[ef68c33fd2f3b212]::check::check_fn
  49:     0x7fa5577d2c49 - <rustc_hir_typeck[ef68c33fd2f3b212]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[ef68c33fd2f3b212]::typeck_with_fallback<rustc_hir_typeck[ef68c33fd2f3b212]::typeck::{closure#0}>::{closure#0}::{closure#1}, &rustc_middle[86905b0aa7050cb2]::ty::typeck_results::TypeckResults>
  50:     0x7fa55780639f - rustc_hir_typeck[ef68c33fd2f3b212]::typeck
  51:     0x7fa558f43753 - rustc_query_system[4bcb50d269c32ba]::query::plumbing::try_execute_query::<rustc_query_impl[68facdf58d22355b]::queries::typeck, rustc_query_impl[68facdf58d22355b]::plumbing::QueryCtxt>
  52:     0x7fa558ff9100 - rustc_query_system[4bcb50d269c32ba]::query::plumbing::get_query::<rustc_query_impl[68facdf58d22355b]::queries::typeck, rustc_query_impl[68facdf58d22355b]::plumbing::QueryCtxt, rustc_middle[86905b0aa7050cb2]::dep_graph::dep_node::DepKind>
  53:     0x7fa558c11570 - <rustc_query_impl[68facdf58d22355b]::Queries as rustc_middle[86905b0aa7050cb2]::ty::query::QueryEngine>::typeck
  54:     0x7fa5577c2931 - <core[79021a402131dad8]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[9f90b09327a9efb6]::sync::par_for_each_in<&[rustc_span[fff9630aea768480]::def_id::LocalDefId], <rustc_middle[86905b0aa7050cb2]::hir::map::Map>::par_body_owners<rustc_hir_typeck[ef68c33fd2f3b212]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[79021a402131dad8]::ops::function::FnOnce<()>>::call_once
  55:     0x7fa5578ac196 - std[4fc4c1d6570de2f7]::panicking::try::<(), core[79021a402131dad8]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[9f90b09327a9efb6]::sync::par_for_each_in<&[rustc_span[fff9630aea768480]::def_id::LocalDefId], <rustc_middle[86905b0aa7050cb2]::hir::map::Map>::par_body_owners<rustc_hir_typeck[ef68c33fd2f3b212]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  56:     0x7fa557773f4d - rustc_data_structures[9f90b09327a9efb6]::sync::par_for_each_in::<&[rustc_span[fff9630aea768480]::def_id::LocalDefId], <rustc_middle[86905b0aa7050cb2]::hir::map::Map>::par_body_owners<rustc_hir_typeck[ef68c33fd2f3b212]::typeck_item_bodies::{closure#0}>::{closure#0}>
  57:     0x7fa557805377 - rustc_hir_typeck[ef68c33fd2f3b212]::typeck_item_bodies
  58:     0x7fa558efdb91 - rustc_query_system[4bcb50d269c32ba]::query::plumbing::try_execute_query::<rustc_query_impl[68facdf58d22355b]::queries::typeck_item_bodies, rustc_query_impl[68facdf58d22355b]::plumbing::QueryCtxt>
  59:     0x7fa558fc94df - rustc_query_system[4bcb50d269c32ba]::query::plumbing::get_query::<rustc_query_impl[68facdf58d22355b]::queries::typeck_item_bodies, rustc_query_impl[68facdf58d22355b]::plumbing::QueryCtxt, rustc_middle[86905b0aa7050cb2]::dep_graph::dep_node::DepKind>
  60:     0x7fa558c10e0a - <rustc_query_impl[68facdf58d22355b]::Queries as rustc_middle[86905b0aa7050cb2]::ty::query::QueryEngine>::typeck_item_bodies
  61:     0x7fa55791195b - <rustc_session[fad313e02bf16eae]::session::Session>::time::<(), rustc_hir_analysis[41a3122a3c560e22]::check_crate::{closure#7}>
  62:     0x7fa557969be3 - rustc_hir_analysis[41a3122a3c560e22]::check_crate
  63:     0x7fa5572c8d23 - rustc_interface[37bd6786a70ad224]::passes::analysis
  64:     0x7fa558f45a4e - rustc_query_system[4bcb50d269c32ba]::query::plumbing::try_execute_query::<rustc_query_impl[68facdf58d22355b]::queries::analysis, rustc_query_impl[68facdf58d22355b]::plumbing::QueryCtxt>
  65:     0x7fa558ff9391 - rustc_query_system[4bcb50d269c32ba]::query::plumbing::get_query::<rustc_query_impl[68facdf58d22355b]::queries::analysis, rustc_query_impl[68facdf58d22355b]::plumbing::QueryCtxt, rustc_middle[86905b0aa7050cb2]::dep_graph::dep_node::DepKind>
  66:     0x7fa558be849a - <rustc_query_impl[68facdf58d22355b]::Queries as rustc_middle[86905b0aa7050cb2]::ty::query::QueryEngine>::analysis
  67:     0x7fa5571aa8fc - <rustc_interface[37bd6786a70ad224]::passes::QueryContext>::enter::<rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>
  68:     0x7fa55715241d - <rustc_interface[37bd6786a70ad224]::queries::QueryResult<rustc_interface[37bd6786a70ad224]::passes::QueryContext>>::enter::<core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>, rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}::{closure#2}::{closure#2}>
  69:     0x7fa5571c5ecd - <rustc_interface[37bd6786a70ad224]::interface::Compiler>::enter::<rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}::{closure#2}, core[79021a402131dad8]::result::Result<core[79021a402131dad8]::option::Option<rustc_interface[37bd6786a70ad224]::queries::Linker>, rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>
  70:     0x7fa55713dedc - rustc_span[fff9630aea768480]::with_source_map::<core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>, rustc_interface[37bd6786a70ad224]::interface::run_compiler<core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>, rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  71:     0x7fa5571b82aa - <scoped_tls[76faafc9a8b3b4a9]::ScopedKey<rustc_span[fff9630aea768480]::SessionGlobals>>::set::<rustc_interface[37bd6786a70ad224]::interface::run_compiler<core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>, rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}>::{closure#0}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>
  72:     0x7fa5571b1cb9 - std[4fc4c1d6570de2f7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[37bd6786a70ad224]::util::run_in_thread_pool_with_globals<rustc_interface[37bd6786a70ad224]::interface::run_compiler<core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>, rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}>::{closure#0}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>
  73:     0x7fa5571ab868 - std[4fc4c1d6570de2f7]::panic::catch_unwind::<core[79021a402131dad8]::panic::unwind_safe::AssertUnwindSafe<<std[4fc4c1d6570de2f7]::thread::Builder>::spawn_unchecked_<rustc_interface[37bd6786a70ad224]::util::run_in_thread_pool_with_globals<rustc_interface[37bd6786a70ad224]::interface::run_compiler<core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>, rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}>::{closure#0}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>
  74:     0x7fa557161067 - <<std[4fc4c1d6570de2f7]::thread::Builder>::spawn_unchecked_<rustc_interface[37bd6786a70ad224]::util::run_in_thread_pool_with_globals<rustc_interface[37bd6786a70ad224]::interface::run_compiler<core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>, rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}>::{closure#0}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>::{closure#1} as core[79021a402131dad8]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  75:     0x7fa5566eeffe - std::sys::unix::thread::Thread::new::thread_start::h602b01630ceb10ed
  76:     0x7fa556483b43 - <unknown>
  77:     0x7fa556515a00 - <unknown>
  78:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.68.0-nightly (284de10f4 2023-01-17) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `<Bar as Foo>::N == 3`
#1 [typeck] type-checking `main`
#2 [typeck_item_bodies] type-checking all item bodies
#3 [analysis] running analysis passes on this crate
error: aborting due to previous error
------------------------------------------



---- [ui] checkout/tests/ui/associated-consts/issue-105330.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/associated-consts/issue-105330.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-105330" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-105330/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected one of `!` or `::`, found `A`
   |
   |
LL | impl TraitWAssocConst for impl Demo { //~ ERROR E0404
   |                                     - while parsing this item list starting here
LL |     //~^ ERROR E0562
LL |     pubconst A: str = 32; //~ ERROR expected one of
   |              ^ expected one of `!` or `::`
LL | }
   | - the item list ends here
error[E0404]: expected trait, found struct `Demo`
  --> /checkout/tests/ui/associated-consts/issue-105330.rs:6:32
   |
   |
LL | impl TraitWAssocConst for impl Demo { //~ ERROR E0404

error[E0658]: associated const equality is incomplete
  --> /checkout/tests/ui/associated-consts/issue-105330.rs:11:28
   |
   |
LL | fn foo<A: TraitWAssocConst<A=32>>() { //~ ERROR E0658
   |
   = note: see issue #92827 <https://github.com/rust-lang/rust/issues/92827> for more information
   = note: see issue #92827 <https://github.com/rust-lang/rust/issues/92827> for more information
   = help: add `#![feature(associated_const_equality)]` to the crate attributes to enable
error[E0658]: associated const equality is incomplete
  --> /checkout/tests/ui/associated-consts/issue-105330.rs:17:29
   |
   |
LL | fn main<A: TraitWAssocConst<A=32>>() { //~ ERROR E0131
   |
   = note: see issue #92827 <https://github.com/rust-lang/rust/issues/92827> for more information
   = note: see issue #92827 <https://github.com/rust-lang/rust/issues/92827> for more information
   = help: add `#![feature(associated_const_equality)]` to the crate attributes to enable

error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in type
   |
   |
LL | impl TraitWAssocConst for impl Demo { //~ ERROR E0404


error: internal compiler error: compiler/rustc_middle/src/ty/sty.rs:1264:21: unexpected DefKind in ProjectionTy: AssocConst
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1609:9
stack backtrace:
   0:     0x7f072d552825 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h38611d6c58ff15d7
   1:     0x7f072d5c2d18 - core::fmt::write::h2d75faa94db0a514
   1:     0x7f072d5c2d18 - core::fmt::write::h2d75faa94db0a514
   2:     0x7f072d5446d1 - std::io::Write::write_fmt::h00bd98e74899b603
   3:     0x7f072d552631 - std::sys_common::backtrace::print::h805af3402a33b9e0
   4:     0x7f072d555a14 - std::panicking::default_hook::{{closure}}::hd3489b51e502c13f
   5:     0x7f072d5556da - std::panicking::default_hook::hcb306b30123f73b7
   6:     0x7f072dfc4b82 - rustc_driver[24f6a6ae78ae4804]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f072d556184 - std::panicking::rust_panic_with_hook::h87fc5763cb93e40f
   8:     0x7f0730ccaf53 - std[4fc4c1d6570de2f7]::panicking::begin_panic::<rustc_errors[693af92fc8bcbc7a]::ExplicitBug>::{closure#0}
   9:     0x7f0730cca496 - std[4fc4c1d6570de2f7]::sys_common::backtrace::__rust_end_short_backtrace::<std[4fc4c1d6570de2f7]::panicking::begin_panic<rustc_errors[693af92fc8bcbc7a]::ExplicitBug>::{closure#0}, !>
  10:     0x7f072df6b956 - std[4fc4c1d6570de2f7]::panicking::begin_panic::<rustc_errors[693af92fc8bcbc7a]::ExplicitBug>
  11:     0x7f0730e5a826 - std[4fc4c1d6570de2f7]::panic::panic_any::<rustc_errors[693af92fc8bcbc7a]::ExplicitBug>
  12:     0x7f0730e59ce0 - <rustc_errors[693af92fc8bcbc7a]::HandlerInner>::bug::<&alloc[a656fae0f04844bf]::string::String>
  13:     0x7f0730e598a0 - <rustc_errors[693af92fc8bcbc7a]::Handler>::bug::<&alloc[a656fae0f04844bf]::string::String>
  14:     0x7f0730eab775 - rustc_middle[86905b0aa7050cb2]::util::bug::opt_span_bug_fmt::<rustc_span[fff9630aea768480]::span_encoding::Span>::{closure#0}
  15:     0x7f0730ea9ffc - rustc_middle[86905b0aa7050cb2]::ty::context::tls::with_opt::<rustc_middle[86905b0aa7050cb2]::util::bug::opt_span_bug_fmt<rustc_span[fff9630aea768480]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  16:     0x7f0730ea9fa6 - rustc_middle[86905b0aa7050cb2]::ty::context::tls::with_context_opt::<rustc_middle[86905b0aa7050cb2]::ty::context::tls::with_opt<rustc_middle[86905b0aa7050cb2]::util::bug::opt_span_bug_fmt<rustc_span[fff9630aea768480]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  17:     0x7f0730eab6b9 - rustc_middle[86905b0aa7050cb2]::util::bug::opt_span_bug_fmt::<rustc_span[fff9630aea768480]::span_encoding::Span>
  18:     0x7f072df6b905 - rustc_middle[86905b0aa7050cb2]::util::bug::bug_fmt
  19:     0x7f0730e0b8ed - <rustc_middle[86905b0aa7050cb2]::ty::sty::AliasTy>::trait_def_id
  20:     0x7f0730e0bc1f - <rustc_middle[86905b0aa7050cb2]::ty::sty::AliasTy>::trait_ref
  21:     0x7f0730915e0a - rustc_trait_selection[ce79bbcd8c7b125d]::traits::project::opt_normalize_projection_type
  22:     0x7f073090c7b5 - rustc_trait_selection[ce79bbcd8c7b125d]::traits::project::project_and_unify_type
  23:     0x7f07308dfe70 - <rustc_infer[965322cfb50022e7]::infer::InferCtxt>::commit_if_ok::<rustc_trait_selection[ce79bbcd8c7b125d]::traits::project::ProjectAndUnifyResult, rustc_infer[965322cfb50022e7]::traits::project::MismatchedProjectionTypes, rustc_trait_selection[ce79bbcd8c7b125d]::traits::project::poly_project_and_unify_type::{closure#0}>
  24:     0x7f073090c326 - rustc_trait_selection[ce79bbcd8c7b125d]::traits::project::poly_project_and_unify_type
  25:     0x7f0730a2493a - <rustc_trait_selection[ce79bbcd8c7b125d]::traits::select::SelectionContext>::evaluate_predicate_recursively
  26:     0x7f07308e6273 - <rustc_infer[965322cfb50022e7]::infer::InferCtxt>::probe::<core[79021a402131dad8]::result::Result<rustc_middle[86905b0aa7050cb2]::traits::select::EvaluationResult, rustc_middle[86905b0aa7050cb2]::traits::select::OverflowError>, <rustc_trait_selection[ce79bbcd8c7b125d]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[ce79bbcd8c7b125d]::traits::select::SelectionContext>::evaluate_root_obligation::{closure#0}>::{closure#0}>
  27:     0x7f0730a08b7d - <rustc_trait_selection[ce79bbcd8c7b125d]::traits::select::SelectionContext>::evaluate_root_obligation
  28:     0x7f072f675acf - rustc_traits[d705a9fda77a3464]::evaluate_obligation::evaluate_obligation
  29:     0x7f072fe3e10d - rustc_query_system[4bcb50d269c32ba]::query::plumbing::get_query::<rustc_query_impl[68facdf58d22355b]::queries::evaluate_obligation, rustc_query_impl[68facdf58d22355b]::plumbing::QueryCtxt, rustc_middle[86905b0aa7050cb2]::dep_graph::dep_node::DepKind>
  30:     0x7f072facd538 - <rustc_query_impl[68facdf58d22355b]::Queries as rustc_middle[86905b0aa7050cb2]::ty::query::QueryEngine>::evaluate_obligation
  31:     0x7f07308f5f8a - <rustc_infer[965322cfb50022e7]::infer::InferCtxt as rustc_trait_selection[ce79bbcd8c7b125d]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation
  32:     0x7f07308f612c - <rustc_infer[965322cfb50022e7]::infer::InferCtxt as rustc_trait_selection[ce79bbcd8c7b125d]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation_no_overflow
  33:     0x7f0730a7745f - <rustc_trait_selection[ce79bbcd8c7b125d]::traits::fulfill::FulfillProcessor>::process_projection_obligation
  34:     0x7f0730a8180f - <rustc_trait_selection[ce79bbcd8c7b125d]::traits::fulfill::FulfillProcessor as rustc_data_structures[9f90b09327a9efb6]::obligation_forest::ObligationProcessor>::process_obligation
  35:     0x7f07309536a3 - <rustc_data_structures[9f90b09327a9efb6]::obligation_forest::ObligationForest<rustc_trait_selection[ce79bbcd8c7b125d]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[ce79bbcd8c7b125d]::traits::fulfill::FulfillProcessor>
  36:     0x7f0730a76ba7 - <rustc_trait_selection[ce79bbcd8c7b125d]::traits::fulfill::FulfillmentContext as rustc_infer[965322cfb50022e7]::traits::engine::TraitEngine>::select_where_possible
  37:     0x7f072e50ddd5 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_argument_types
  38:     0x7f072e4dc991 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::confirm_builtin_call
  39:     0x7f072e4db71a - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_call
  40:     0x7f072e55fa92 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_expr_kind
  41:     0x7f072e4f5a71 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  42:     0x7f072e55e8d2 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  43:     0x7f072e4da9a0 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_call
  44:     0x7f072e55fa92 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_expr_kind
  45:     0x7f072e4f5a71 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  46:     0x7f072e55e8d2 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  47:     0x7f072e515b01 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_stmt
  48:     0x7f072e516067 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_block_with_expected
  49:     0x7f072e55fdd4 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_expr_kind
  50:     0x7f072e4f5a71 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  51:     0x7f072e55e8d2 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  52:     0x7f072e4f7b9b - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_return_expr
  53:     0x7f072e64c997 - rustc_hir_typeck[ef68c33fd2f3b212]::check::check_fn
  54:     0x7f072e646c49 - <rustc_hir_typeck[ef68c33fd2f3b212]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[ef68c33fd2f3b212]::typeck_with_fallback<rustc_hir_typeck[ef68c33fd2f3b212]::typeck::{closure#0}>::{closure#0}::{closure#1}, &rustc_middle[86905b0aa7050cb2]::ty::typeck_results::TypeckResults>
  55:     0x7f072e67a39f - rustc_hir_typeck[ef68c33fd2f3b212]::typeck
  56:     0x7f072fdb7753 - rustc_query_system[4bcb50d269c32ba]::query::plumbing::try_execute_query::<rustc_query_impl[68facdf58d22355b]::queries::typeck, rustc_query_impl[68facdf58d22355b]::plumbing::QueryCtxt>
  57:     0x7f072fe6d100 - rustc_query_system[4bcb50d269c32ba]::query::plumbing::get_query::<rustc_query_impl[68facdf58d22355b]::queries::typeck, rustc_query_impl[68facdf58d22355b]::plumbing::QueryCtxt, rustc_middle[86905b0aa7050cb2]::dep_graph::dep_node::DepKind>
  58:     0x7f072fa85570 - <rustc_query_impl[68facdf58d22355b]::Queries as rustc_middle[86905b0aa7050cb2]::ty::query::QueryEngine>::typeck
  59:     0x7f072e636931 - <core[79021a402131dad8]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[9f90b09327a9efb6]::sync::par_for_each_in<&[rustc_span[fff9630aea768480]::def_id::LocalDefId], <rustc_middle[86905b0aa7050cb2]::hir::map::Map>::par_body_owners<rustc_hir_typeck[ef68c33fd2f3b212]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[79021a402131dad8]::ops::function::FnOnce<()>>::call_once
  60:     0x7f072e720196 - std[4fc4c1d6570de2f7]::panicking::try::<(), core[79021a402131dad8]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[9f90b09327a9efb6]::sync::par_for_each_in<&[rustc_span[fff9630aea768480]::def_id::LocalDefId], <rustc_middle[86905b0aa7050cb2]::hir::map::Map>::par_body_owners<rustc_hir_typeck[ef68c33fd2f3b212]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  61:     0x7f072e5e7f4d - rustc_data_structures[9f90b09327a9efb6]::sync::par_for_each_in::<&[rustc_span[fff9630aea768480]::def_id::LocalDefId], <rustc_middle[86905b0aa7050cb2]::hir::map::Map>::par_body_owners<rustc_hir_typeck[ef68c33fd2f3b212]::typeck_item_bodies::{closure#0}>::{closure#0}>
  62:     0x7f072e679377 - rustc_hir_typeck[ef68c33fd2f3b212]::typeck_item_bodies
  63:     0x7f072fd71b91 - rustc_query_system[4bcb50d269c32ba]::query::plumbing::try_execute_query::<rustc_query_impl[68facdf58d22355b]::queries::typeck_item_bodies, rustc_query_impl[68facdf58d22355b]::plumbing::QueryCtxt>
  64:     0x7f072fe3d4df - rustc_query_system[4bcb50d269c32ba]::query::plumbing::get_query::<rustc_query_impl[68facdf58d22355b]::queries::typeck_item_bodies, rustc_query_impl[68facdf58d22355b]::plumbing::QueryCtxt, rustc_middle[86905b0aa7050cb2]::dep_graph::dep_node::DepKind>
  65:     0x7f072fa84e0a - <rustc_query_impl[68facdf58d22355b]::Queries as rustc_middle[86905b0aa7050cb2]::ty::query::QueryEngine>::typeck_item_bodies
  66:     0x7f072e78595b - <rustc_session[fad313e02bf16eae]::session::Session>::time::<(), rustc_hir_analysis[41a3122a3c560e22]::check_crate::{closure#7}>
  67:     0x7f072e7ddbe3 - rustc_hir_analysis[41a3122a3c560e22]::check_crate
  68:     0x7f072e13cd23 - rustc_interface[37bd6786a70ad224]::passes::analysis
  69:     0x7f072fdb9a4e - rustc_query_system[4bcb50d269c32ba]::query::plumbing::try_execute_query::<rustc_query_impl[68facdf58d22355b]::queries::analysis, rustc_query_impl[68facdf58d22355b]::plumbing::QueryCtxt>
  70:     0x7f072fe6d391 - rustc_query_system[4bcb50d269c32ba]::query::plumbing::get_query::<rustc_query_impl[68facdf58d22355b]::queries::analysis, rustc_query_impl[68facdf58d22355b]::plumbing::QueryCtxt, rustc_middle[86905b0aa7050cb2]::dep_graph::dep_node::DepKind>
  71:     0x7f072fa5c49a - <rustc_query_impl[68facdf58d22355b]::Queries as rustc_middle[86905b0aa7050cb2]::ty::query::QueryEngine>::analysis
  72:     0x7f072e01e8fc - <rustc_interface[37bd6786a70ad224]::passes::QueryContext>::enter::<rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>
  73:     0x7f072dfc641d - <rustc_interface[37bd6786a70ad224]::queries::QueryResult<rustc_interface[37bd6786a70ad224]::passes::QueryContext>>::enter::<core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>, rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}::{closure#2}::{closure#2}>
  74:     0x7f072e039ecd - <rustc_interface[37bd6786a70ad224]::interface::Compiler>::enter::<rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}::{closure#2}, core[79021a402131dad8]::result::Result<core[79021a402131dad8]::option::Option<rustc_interface[37bd6786a70ad224]::queries::Linker>, rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>
  75:     0x7f072dfb1edc - rustc_span[fff9630aea768480]::with_source_map::<core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>, rustc_interface[37bd6786a70ad224]::interface::run_compiler<core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>, rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  76:     0x7f072e02c2aa - <scoped_tls[76faafc9a8b3b4a9]::ScopedKey<rustc_span[fff9630aea768480]::SessionGlobals>>::set::<rustc_interface[37bd6786a70ad224]::interface::run_compiler<core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>, rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}>::{closure#0}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>
  77:     0x7f072e025cb9 - std[4fc4c1d6570de2f7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[37bd6786a70ad224]::util::run_in_thread_pool_with_globals<rustc_interface[37bd6786a70ad224]::interface::run_compiler<core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>, rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}>::{closure#0}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>
  78:     0x7f072e01f868 - std[4fc4c1d6570de2f7]::panic::catch_unwind::<core[79021a402131dad8]::panic::unwind_safe::AssertUnwindSafe<<std[4fc4c1d6570de2f7]::thread::Builder>::spawn_unchecked_<rustc_interface[37bd6786a70ad224]::util::run_in_thread_pool_with_globals<rustc_interface[37bd6786a70ad224]::interface::run_compiler<core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>, rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}>::{closure#0}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>
  79:     0x7f072dfd5067 - <<std[4fc4c1d6570de2f7]::thread::Builder>::spawn_unchecked_<rustc_interface[37bd6786a70ad224]::util::run_in_thread_pool_with_globals<rustc_interface[37bd6786a70ad224]::interface::run_compiler<core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>, rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}>::{closure#0}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>::{closure#1} as core[79021a402131dad8]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  80:     0x7f072d562ffe - std::sys::unix::thread::Thread::new::thread_start::h602b01630ceb10ed
  81:     0x7f072d2f7b43 - <unknown>
  82:     0x7f072d389a00 - <unknown>
  83:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.68.0-nightly (284de10f4 2023-01-17) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `<Demo as TraitWAssocConst>::A == 32`
#1 [typeck] type-checking `foo`
#2 [typeck_item_bodies] type-checking all item bodies
#3 [analysis] running analysis passes on this crate
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0404, E0562, E0658.
For more information about an error, try `rustc --explain E0404`.
For more information about an error, try `rustc --explain E0404`.
------------------------------------------


---- [ui] checkout/tests/ui/associated-type-bounds/const-projection-err.rs#stock stdout ----

error in revision `stock`: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/associated-type-bounds/const-projection-err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "stock" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/const-projection-err.stock" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/const-projection-err.stock/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_middle/src/ty/sty.rs:1264:21: unexpected DefKind in ProjectionTy: AssocConst
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1609:9
stack backtrace:
   0:     0x7f15c4954825 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h38611d6c58ff15d7
   1:     0x7f15c49c4d18 - core::fmt::write::h2d75faa94db0a514
   1:     0x7f15c49c4d18 - core::fmt::write::h2d75faa94db0a514
   2:     0x7f15c49466d1 - std::io::Write::write_fmt::h00bd98e74899b603
   3:     0x7f15c4954631 - std::sys_common::backtrace::print::h805af3402a33b9e0
   4:     0x7f15c4957a14 - std::panicking::default_hook::{{closure}}::hd3489b51e502c13f
   5:     0x7f15c49576da - std::panicking::default_hook::hcb306b30123f73b7
   6:     0x7f15c53c6b82 - rustc_driver[24f6a6ae78ae4804]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f15c4958184 - std::panicking::rust_panic_with_hook::h87fc5763cb93e40f
   8:     0x7f15c80ccf53 - std[4fc4c1d6570de2f7]::panicking::begin_panic::<rustc_errors[693af92fc8bcbc7a]::ExplicitBug>::{closure#0}
   9:     0x7f15c80cc496 - std[4fc4c1d6570de2f7]::sys_common::backtrace::__rust_end_short_backtrace::<std[4fc4c1d6570de2f7]::panicking::begin_panic<rustc_errors[693af92fc8bcbc7a]::ExplicitBug>::{closure#0}, !>
  10:     0x7f15c536d956 - std[4fc4c1d6570de2f7]::panicking::begin_panic::<rustc_errors[693af92fc8bcbc7a]::ExplicitBug>
  11:     0x7f15c825c826 - std[4fc4c1d6570de2f7]::panic::panic_any::<rustc_errors[693af92fc8bcbc7a]::ExplicitBug>
  12:     0x7f15c825bce0 - <rustc_errors[693af92fc8bcbc7a]::HandlerInner>::bug::<&alloc[a656fae0f04844bf]::string::String>
  13:     0x7f15c825b8a0 - <rustc_errors[693af92fc8bcbc7a]::Handler>::bug::<&alloc[a656fae0f04844bf]::string::String>
  14:     0x7f15c82ad775 - rustc_middle[86905b0aa7050cb2]::util::bug::opt_span_bug_fmt::<rustc_span[fff9630aea768480]::span_encoding::Span>::{closure#0}
  15:     0x7f15c82abffc - rustc_middle[86905b0aa7050cb2]::ty::context::tls::with_opt::<rustc_middle[86905b0aa7050cb2]::util::bug::opt_span_bug_fmt<rustc_span[fff9630aea768480]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  16:     0x7f15c82abfa6 - rustc_middle[86905b0aa7050cb2]::ty::context::tls::with_context_opt::<rustc_middle[86905b0aa7050cb2]::ty::context::tls::with_opt<rustc_middle[86905b0aa7050cb2]::util::bug::opt_span_bug_fmt<rustc_span[fff9630aea768480]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  17:     0x7f15c82ad6b9 - rustc_middle[86905b0aa7050cb2]::util::bug::opt_span_bug_fmt::<rustc_span[fff9630aea768480]::span_encoding::Span>
  18:     0x7f15c536d905 - rustc_middle[86905b0aa7050cb2]::util::bug::bug_fmt
  19:     0x7f15c820d8ed - <rustc_middle[86905b0aa7050cb2]::ty::sty::AliasTy>::trait_def_id
  20:     0x7f15c820dc1f - <rustc_middle[86905b0aa7050cb2]::ty::sty::AliasTy>::trait_ref
  21:     0x7f15c7d17e0a - rustc_trait_selection[ce79bbcd8c7b125d]::traits::project::opt_normalize_projection_type
  22:     0x7f15c7d0e7b5 - rustc_trait_selection[ce79bbcd8c7b125d]::traits::project::project_and_unify_type
  23:     0x7f15c7ce1e70 - <rustc_infer[965322cfb50022e7]::infer::InferCtxt>::commit_if_ok::<rustc_trait_selection[ce79bbcd8c7b125d]::traits::project::ProjectAndUnifyResult, rustc_infer[965322cfb50022e7]::traits::project::MismatchedProjectionTypes, rustc_trait_selection[ce79bbcd8c7b125d]::traits::project::poly_project_and_unify_type::{closure#0}>
  24:     0x7f15c7d0e326 - rustc_trait_selection[ce79bbcd8c7b125d]::traits::project::poly_project_and_unify_type
  25:     0x7f15c7e793ea - <rustc_trait_selection[ce79bbcd8c7b125d]::traits::fulfill::FulfillProcessor>::process_projection_obligation
  26:     0x7f15c7e8380f - <rustc_trait_selection[ce79bbcd8c7b125d]::traits::fulfill::FulfillProcessor as rustc_data_structures[9f90b09327a9efb6]::obligation_forest::ObligationProcessor>::process_obligation
  27:     0x7f15c7d556a3 - <rustc_data_structures[9f90b09327a9efb6]::obligation_forest::ObligationForest<rustc_trait_selection[ce79bbcd8c7b125d]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[ce79bbcd8c7b125d]::traits::fulfill::FulfillProcessor>
  28:     0x7f15c7e78ba7 - <rustc_trait_selection[ce79bbcd8c7b125d]::traits::fulfill::FulfillmentContext as rustc_infer[965322cfb50022e7]::traits::engine::TraitEngine>::select_where_possible
  29:     0x7f15c590fdd5 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_argument_types
  30:     0x7f15c58de991 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::confirm_builtin_call
  31:     0x7f15c58dd71a - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_call
  32:     0x7f15c5961a92 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_expr_kind
  33:     0x7f15c58f7a71 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  34:     0x7f15c59608d2 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  35:     0x7f15c5917b01 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_stmt
  36:     0x7f15c5918067 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_block_with_expected
  37:     0x7f15c5961dd4 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_expr_kind
  38:     0x7f15c58f7a71 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  39:     0x7f15c59608d2 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  40:     0x7f15c58f9b9b - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_return_expr
  41:     0x7f15c5a4e997 - rustc_hir_typeck[ef68c33fd2f3b212]::check::check_fn
  42:     0x7f15c5a48c49 - <rustc_hir_typeck[ef68c33fd2f3b212]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[ef68c33fd2f3b212]::typeck_with_fallback<rustc_hir_typeck[ef68c33fd2f3b212]::typeck::{closure#0}>::{closure#0}::{closure#1}, &rustc_middle[86905b0aa7050cb2]::ty::typeck_results::TypeckResults>
  43:     0x7f15c5a7c39f - rustc_hir_typeck[ef68c33fd2f3b212]::typeck
  44:     0x7f15c71b9753 - rustc_query_system[4bcb50d269c32ba]::query::plumbing::try_execute_query::<rustc_query_impl[68facdf58d22355b]::queries::typeck, rustc_query_impl[68facdf58d22355b]::plumbing::QueryCtxt>
  45:     0x7f15c726f100 - rustc_query_system[4bcb50d269c32ba]::query::plumbing::get_query::<rustc_query_impl[68facdf58d22355b]::queries::typeck, rustc_query_impl[68facdf58d22355b]::plumbing::QueryCtxt, rustc_middle[86905b0aa7050cb2]::dep_graph::dep_node::DepKind>
  46:     0x7f15c6e87570 - <rustc_query_impl[68facdf58d22355b]::Queries as rustc_middle[86905b0aa7050cb2]::ty::query::QueryEngine>::typeck
  47:     0x7f15c5a38931 - <core[79021a402131dad8]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[9f90b09327a9efb6]::sync::par_for_each_in<&[rustc_span[fff9630aea768480]::def_id::LocalDefId], <rustc_middle[86905b0aa7050cb2]::hir::map::Map>::par_body_owners<rustc_hir_typeck[ef68c33fd2f3b212]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[79021a402131dad8]::ops::function::FnOnce<()>>::call_once
  48:     0x7f15c5b22196 - std[4fc4c1d6570de2f7]::panicking::try::<(), core[79021a402131dad8]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[9f90b09327a9efb6]::sync::par_for_each_in<&[rustc_span[fff9630aea768480]::def_id::LocalDefId], <rustc_middle[86905b0aa7050cb2]::hir::map::Map>::par_body_owners<rustc_hir_typeck[ef68c33fd2f3b212]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  49:     0x7f15c59e9f4d - rustc_data_structures[9f90b09327a9efb6]::sync::par_for_each_in::<&[rustc_span[fff9630aea768480]::def_id::LocalDefId], <rustc_middle[86905b0aa7050cb2]::hir::map::Map>::par_body_owners<rustc_hir_typeck[ef68c33fd2f3b212]::typeck_item_bodies::{closure#0}>::{closure#0}>
  50:     0x7f15c5a7b377 - rustc_hir_typeck[ef68c33fd2f3b212]::typeck_item_bodies
  51:     0x7f15c7173b91 - rustc_query_system[4bcb50d269c32ba]::query::plumbing::try_execute_query::<rustc_query_impl[68facdf58d22355b]::queries::typeck_item_bodies, rustc_query_impl[68facdf58d22355b]::plumbing::QueryCtxt>
  52:     0x7f15c723f4df - rustc_query_system[4bcb50d269c32ba]::query::plumbing::get_query::<rustc_query_impl[68facdf58d22355b]::queries::typeck_item_bodies, rustc_query_impl[68facdf58d22355b]::plumbing::QueryCtxt, rustc_middle[86905b0aa7050cb2]::dep_graph::dep_node::DepKind>
  53:     0x7f15c6e86e0a - <rustc_query_impl[68facdf58d22355b]::Queries as rustc_middle[86905b0aa7050cb2]::ty::query::QueryEngine>::typeck_item_bodies
  54:     0x7f15c5b8795b - <rustc_session[fad313e02bf16eae]::session::Session>::time::<(), rustc_hir_analysis[41a3122a3c560e22]::check_crate::{closure#7}>
  55:     0x7f15c5bdfbe3 - rustc_hir_analysis[41a3122a3c560e22]::check_crate
  56:     0x7f15c553ed23 - rustc_interface[37bd6786a70ad224]::passes::analysis
  57:     0x7f15c71bba4e - rustc_query_system[4bcb50d269c32ba]::query::plumbing::try_execute_query::<rustc_query_impl[68facdf58d22355b]::queries::analysis, rustc_query_impl[68facdf58d22355b]::plumbing::QueryCtxt>
  58:     0x7f15c726f391 - rustc_query_system[4bcb50d269c32ba]::query::plumbing::get_query::<rustc_query_impl[68facdf58d22355b]::queries::analysis, rustc_query_impl[68facdf58d22355b]::plumbing::QueryCtxt, rustc_middle[86905b0aa7050cb2]::dep_graph::dep_node::DepKind>
  59:     0x7f15c6e5e49a - <rustc_query_impl[68facdf58d22355b]::Queries as rustc_middle[86905b0aa7050cb2]::ty::query::QueryEngine>::analysis
  60:     0x7f15c54208fc - <rustc_interface[37bd6786a70ad224]::passes::QueryContext>::enter::<rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>
  61:     0x7f15c53c841d - <rustc_interface[37bd6786a70ad224]::queries::QueryResult<rustc_interface[37bd6786a70ad224]::passes::QueryContext>>::enter::<core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>, rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}::{closure#2}::{closure#2}>
  62:     0x7f15c543becd - <rustc_interface[37bd6786a70ad224]::interface::Compiler>::enter::<rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}::{closure#2}, core[79021a402131dad8]::result::Result<core[79021a402131dad8]::option::Option<rustc_interface[37bd6786a70ad224]::queries::Linker>, rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>
  63:     0x7f15c53b3edc - rustc_span[fff9630aea768480]::with_source_map::<core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>, rustc_interface[37bd6786a70ad224]::interface::run_compiler<core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>, rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  64:     0x7f15c542e2aa - <scoped_tls[76faafc9a8b3b4a9]::ScopedKey<rustc_span[fff9630aea768480]::SessionGlobals>>::set::<rustc_interface[37bd6786a70ad224]::interface::run_compiler<core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>, rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}>::{closure#0}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>
  65:     0x7f15c5427cb9 - std[4fc4c1d6570de2f7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[37bd6786a70ad224]::util::run_in_thread_pool_with_globals<rustc_interface[37bd6786a70ad224]::interface::run_compiler<core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>, rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}>::{closure#0}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>
  66:     0x7f15c5421868 - std[4fc4c1d6570de2f7]::panic::catch_unwind::<core[79021a402131dad8]::panic::unwind_safe::AssertUnwindSafe<<std[4fc4c1d6570de2f7]::thread::Builder>::spawn_unchecked_<rustc_interface[37bd6786a70ad224]::util::run_in_thread_pool_with_globals<rustc_interface[37bd6786a70ad224]::interface::run_compiler<core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>, rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}>::{closure#0}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>
  67:     0x7f15c53d7067 - <<std[4fc4c1d6570de2f7]::thread::Builder>::spawn_unchecked_<rustc_interface[37bd6786a70ad224]::util::run_in_thread_pool_with_globals<rustc_interface[37bd6786a70ad224]::interface::run_compiler<core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>, rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}>::{closure#0}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>::{closure#1} as core[79021a402131dad8]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  68:     0x7f15c4964ffe - std::sys::unix::thread::Thread::new::thread_start::h602b01630ceb10ed
  69:     0x7f15c46f9b43 - <unknown>
  70:     0x7f15c478ba00 - <unknown>
  71:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.68.0-nightly (284de10f4 2023-01-17) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `bar`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
error: aborting due to previous error
------------------------------------------



---- [ui] checkout/tests/ui/associated-type-bounds/const-projection-err.rs#gce stdout ----

error in revision `gce`: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/associated-type-bounds/const-projection-err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "gce" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/const-projection-err.gce" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/const-projection-err.gce/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `generic_const_exprs` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![cfg_attr(gce, feature(generic_const_exprs))]
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = note: `#[warn(incomplete_features)]` on by default


error: internal compiler error: compiler/rustc_middle/src/ty/sty.rs:1264:21: unexpected DefKind in ProjectionTy: AssocConst
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1609:9
stack backtrace:
   0:     0x7f893f4c0825 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h38611d6c58ff15d7
   1:     0x7f893f530d18 - core::fmt::write::h2d75faa94db0a514
   1:     0x7f893f530d18 - core::fmt::write::h2d75faa94db0a514
   2:     0x7f893f4b26d1 - std::io::Write::write_fmt::h00bd98e74899b603
   3:     0x7f893f4c0631 - std::sys_common::backtrace::print::h805af3402a33b9e0
   4:     0x7f893f4c3a14 - std::panicking::default_hook::{{closure}}::hd3489b51e502c13f
   5:     0x7f893f4c36da - std::panicking::default_hook::hcb306b30123f73b7
   6:     0x7f893ff32b82 - rustc_driver[24f6a6ae78ae4804]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f893f4c4184 - std::panicking::rust_panic_with_hook::h87fc5763cb93e40f
   8:     0x7f8942c38f53 - std[4fc4c1d6570de2f7]::panicking::begin_panic::<rustc_errors[693af92fc8bcbc7a]::ExplicitBug>::{closure#0}
   9:     0x7f8942c38496 - std[4fc4c1d6570de2f7]::sys_common::backtrace::__rust_end_short_backtrace::<std[4fc4c1d6570de2f7]::panicking::begin_panic<rustc_errors[693af92fc8bcbc7a]::ExplicitBug>::{closure#0}, !>
  10:     0x7f893fed9956 - std[4fc4c1d6570de2f7]::panicking::begin_panic::<rustc_errors[693af92fc8bcbc7a]::ExplicitBug>
  11:     0x7f8942dc8826 - std[4fc4c1d6570de2f7]::panic::panic_any::<rustc_errors[693af92fc8bcbc7a]::ExplicitBug>
  12:     0x7f8942dc7ce0 - <rustc_errors[693af92fc8bcbc7a]::HandlerInner>::bug::<&alloc[a656fae0f04844bf]::string::String>
  13:     0x7f8942dc78a0 - <rustc_errors[693af92fc8bcbc7a]::Handler>::bug::<&alloc[a656fae0f04844bf]::string::String>
  14:     0x7f8942e19775 - rustc_middle[86905b0aa7050cb2]::util::bug::opt_span_bug_fmt::<rustc_span[fff9630aea768480]::span_encoding::Span>::{closure#0}
  15:     0x7f8942e17ffc - rustc_middle[86905b0aa7050cb2]::ty::context::tls::with_opt::<rustc_middle[86905b0aa7050cb2]::util::bug::opt_span_bug_fmt<rustc_span[fff9630aea768480]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  16:     0x7f8942e17fa6 - rustc_middle[86905b0aa7050cb2]::ty::context::tls::with_context_opt::<rustc_middle[86905b0aa7050cb2]::ty::context::tls::with_opt<rustc_middle[86905b0aa7050cb2]::util::bug::opt_span_bug_fmt<rustc_span[fff9630aea768480]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  17:     0x7f8942e196b9 - rustc_middle[86905b0aa7050cb2]::util::bug::opt_span_bug_fmt::<rustc_span[fff9630aea768480]::span_encoding::Span>
  18:     0x7f893fed9905 - rustc_middle[86905b0aa7050cb2]::util::bug::bug_fmt
  19:     0x7f8942d798ed - <rustc_middle[86905b0aa7050cb2]::ty::sty::AliasTy>::trait_def_id
  20:     0x7f8942d79c1f - <rustc_middle[86905b0aa7050cb2]::ty::sty::AliasTy>::trait_ref
  21:     0x7f8942883e0a - rustc_trait_selection[ce79bbcd8c7b125d]::traits::project::opt_normalize_projection_type
  22:     0x7f894287a7b5 - rustc_trait_selection[ce79bbcd8c7b125d]::traits::project::project_and_unify_type
  23:     0x7f894284de70 - <rustc_infer[965322cfb50022e7]::infer::InferCtxt>::commit_if_ok::<rustc_trait_selection[ce79bbcd8c7b125d]::traits::project::ProjectAndUnifyResult, rustc_infer[965322cfb50022e7]::traits::project::MismatchedProjectionTypes, rustc_trait_selection[ce79bbcd8c7b125d]::traits::project::poly_project_and_unify_type::{closure#0}>
  24:     0x7f894287a326 - rustc_trait_selection[ce79bbcd8c7b125d]::traits::project::poly_project_and_unify_type
  25:     0x7f89429e53ea - <rustc_trait_selection[ce79bbcd8c7b125d]::traits::fulfill::FulfillProcessor>::process_projection_obligation
  26:     0x7f89429ef80f - <rustc_trait_selection[ce79bbcd8c7b125d]::traits::fulfill::FulfillProcessor as rustc_data_structures[9f90b09327a9efb6]::obligation_forest::ObligationProcessor>::process_obligation
  27:     0x7f89428c16a3 - <rustc_data_structures[9f90b09327a9efb6]::obligation_forest::ObligationForest<rustc_trait_selection[ce79bbcd8c7b125d]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[ce79bbcd8c7b125d]::traits::fulfill::FulfillProcessor>
  28:     0x7f89429e4ba7 - <rustc_trait_selection[ce79bbcd8c7b125d]::traits::fulfill::FulfillmentContext as rustc_infer[965322cfb50022e7]::traits::engine::TraitEngine>::select_where_possible
  29:     0x7f894047bdd5 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_argument_types
  30:     0x7f894044a991 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::confirm_builtin_call
  31:     0x7f894044971a - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_call
  32:     0x7f89404cda92 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_expr_kind
  33:     0x7f8940463a71 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  34:     0x7f89404cc8d2 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  35:     0x7f8940483b01 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_stmt
  36:     0x7f8940484067 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_block_with_expected
  37:     0x7f89404cddd4 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_expr_kind
  38:     0x7f8940463a71 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  39:     0x7f89404cc8d2 - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  40:     0x7f8940465b9b - <rustc_hir_typeck[ef68c33fd2f3b212]::fn_ctxt::FnCtxt>::check_return_expr
  41:     0x7f89405ba997 - rustc_hir_typeck[ef68c33fd2f3b212]::check::check_fn
  42:     0x7f89405b4c49 - <rustc_hir_typeck[ef68c33fd2f3b212]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[ef68c33fd2f3b212]::typeck_with_fallback<rustc_hir_typeck[ef68c33fd2f3b212]::typeck::{closure#0}>::{closure#0}::{closure#1}, &rustc_middle[86905b0aa7050cb2]::ty::typeck_results::TypeckResults>
  43:     0x7f89405e839f - rustc_hir_typeck[ef68c33fd2f3b212]::typeck
  44:     0x7f8941d25753 - rustc_query_system[4bcb50d269c32ba]::query::plumbing::try_execute_query::<rustc_query_impl[68facdf58d22355b]::queries::typeck, rustc_query_impl[68facdf58d22355b]::plumbing::QueryCtxt>
  45:     0x7f8941ddb100 - rustc_query_system[4bcb50d269c32ba]::query::plumbing::get_query::<rustc_query_impl[68facdf58d22355b]::queries::typeck, rustc_query_impl[68facdf58d22355b]::plumbing::QueryCtxt, rustc_middle[86905b0aa7050cb2]::dep_graph::dep_node::DepKind>
  46:     0x7f89419f3570 - <rustc_query_impl[68facdf58d22355b]::Queries as rustc_middle[86905b0aa7050cb2]::ty::query::QueryEngine>::typeck
  47:     0x7f89405a4931 - <core[79021a402131dad8]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[9f90b09327a9efb6]::sync::par_for_each_in<&[rustc_span[fff9630aea768480]::def_id::LocalDefId], <rustc_middle[86905b0aa7050cb2]::hir::map::Map>::par_body_owners<rustc_hir_typeck[ef68c33fd2f3b212]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[79021a402131dad8]::ops::function::FnOnce<()>>::call_once
  48:     0x7f894068e196 - std[4fc4c1d6570de2f7]::panicking::try::<(), core[79021a402131dad8]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[9f90b09327a9efb6]::sync::par_for_each_in<&[rustc_span[fff9630aea768480]::def_id::LocalDefId], <rustc_middle[86905b0aa7050cb2]::hir::map::Map>::par_body_owners<rustc_hir_typeck[ef68c33fd2f3b212]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  49:     0x7f8940555f4d - rustc_data_structures[9f90b09327a9efb6]::sync::par_for_each_in::<&[rustc_span[fff9630aea768480]::def_id::LocalDefId], <rustc_middle[86905b0aa7050cb2]::hir::map::Map>::par_body_owners<rustc_hir_typeck[ef68c33fd2f3b212]::typeck_item_bodies::{closure#0}>::{closure#0}>
  50:     0x7f89405e7377 - rustc_hir_typeck[ef68c33fd2f3b212]::typeck_item_bodies
  51:     0x7f8941cdfb91 - rustc_query_system[4bcb50d269c32ba]::query::plumbing::try_execute_query::<rustc_query_impl[68facdf58d22355b]::queries::typeck_item_bodies, rustc_query_impl[68facdf58d22355b]::plumbing::QueryCtxt>
  52:     0x7f8941dab4df - rustc_query_system[4bcb50d269c32ba]::query::plumbing::get_query::<rustc_query_impl[68facdf58d22355b]::queries::typeck_item_bodies, rustc_query_impl[68facdf58d22355b]::plumbing::QueryCtxt, rustc_middle[86905b0aa7050cb2]::dep_graph::dep_node::DepKind>
  53:     0x7f89419f2e0a - <rustc_query_impl[68facdf58d22355b]::Queries as rustc_middle[86905b0aa7050cb2]::ty::query::QueryEngine>::typeck_item_bodies
  54:     0x7f89406f395b - <rustc_session[fad313e02bf16eae]::session::Session>::time::<(), rustc_hir_analysis[41a3122a3c560e22]::check_crate::{closure#7}>
  55:     0x7f894074bbe3 - rustc_hir_analysis[41a3122a3c560e22]::check_crate
  56:     0x7f89400aad23 - rustc_interface[37bd6786a70ad224]::passes::analysis
  57:     0x7f8941d27a4e - rustc_query_system[4bcb50d269c32ba]::query::plumbing::try_execute_query::<rustc_query_impl[68facdf58d22355b]::queries::analysis, rustc_query_impl[68facdf58d22355b]::plumbing::QueryCtxt>
  58:     0x7f8941ddb391 - rustc_query_system[4bcb50d269c32ba]::query::plumbing::get_query::<rustc_query_impl[68facdf58d22355b]::queries::analysis, rustc_query_impl[68facdf58d22355b]::plumbing::QueryCtxt, rustc_middle[86905b0aa7050cb2]::dep_graph::dep_node::DepKind>
  59:     0x7f89419ca49a - <rustc_query_impl[68facdf58d22355b]::Queries as rustc_middle[86905b0aa7050cb2]::ty::query::QueryEngine>::analysis
  60:     0x7f893ff8c8fc - <rustc_interface[37bd6786a70ad224]::passes::QueryContext>::enter::<rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>
  61:     0x7f893ff3441d - <rustc_interface[37bd6786a70ad224]::queries::QueryResult<rustc_interface[37bd6786a70ad224]::passes::QueryContext>>::enter::<core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>, rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}::{closure#2}::{closure#2}>
  62:     0x7f893ffa7ecd - <rustc_interface[37bd6786a70ad224]::interface::Compiler>::enter::<rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}::{closure#2}, core[79021a402131dad8]::result::Result<core[79021a402131dad8]::option::Option<rustc_interface[37bd6786a70ad224]::queries::Linker>, rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>
  63:     0x7f893ff1fedc - rustc_span[fff9630aea768480]::with_source_map::<core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>, rustc_interface[37bd6786a70ad224]::interface::run_compiler<core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>, rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  64:     0x7f893ff9a2aa - <scoped_tls[76faafc9a8b3b4a9]::ScopedKey<rustc_span[fff9630aea768480]::SessionGlobals>>::set::<rustc_interface[37bd6786a70ad224]::interface::run_compiler<core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>, rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}>::{closure#0}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>
  65:     0x7f893ff93cb9 - std[4fc4c1d6570de2f7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[37bd6786a70ad224]::util::run_in_thread_pool_with_globals<rustc_interface[37bd6786a70ad224]::interface::run_compiler<core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>, rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}>::{closure#0}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>
  66:     0x7f893ff8d868 - std[4fc4c1d6570de2f7]::panic::catch_unwind::<core[79021a402131dad8]::panic::unwind_safe::AssertUnwindSafe<<std[4fc4c1d6570de2f7]::thread::Builder>::spawn_unchecked_<rustc_interface[37bd6786a70ad224]::util::run_in_thread_pool_with_globals<rustc_interface[37bd6786a70ad224]::interface::run_compiler<core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>, rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}>::{closure#0}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>
  67:     0x7f893ff43067 - <<std[4fc4c1d6570de2f7]::thread::Builder>::spawn_unchecked_<rustc_interface[37bd6786a70ad224]::util::run_in_thread_pool_with_globals<rustc_interface[37bd6786a70ad224]::interface::run_compiler<core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>, rustc_driver[24f6a6ae78ae4804]::run_compiler::{closure#1}>::{closure#0}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[79021a402131dad8]::result::Result<(), rustc_errors[693af92fc8bcbc7a]::ErrorGuaranteed>>::{closure#1} as core[79021a402131dad8]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  68:     0x7f893f4d0ffe - std::sys::unix::thread::Thread::new::thread_start::h602b01630ceb10ed
  69:     0x7f893f265b43 - <unknown>
  70:     0x7f893f2f7a00 - <unknown>
  71:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.68.0-nightly (284de10f4 2023-01-17) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `bar`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
error: aborting due to previous error; 1 warning emitted
------------------------------------------



---- [ui] checkout/tests/ui/feature-gates/feature-gate-associated_const_equality.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/feature-gates/feature-gate-associated_const_equality.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-associated_const_equality" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-associated_const_equality/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: associated const equality is incomplete
   |
   |
LL | fn foo<A: TraitWAssocConst<A=32>>() {}
   |
   = note: see issue #92827 <https://github.com/rust-lang/rust/issues/92827> for more information
   = note: see issue #92827 <https://github.com/rust-lang/rust/issues/92827> for more information
   = help: add `#![feature(associated_const_equality)]` to the crate attributes to enable

error: internal compiler error: compiler/rustc_middle/src/ty/sty.rs:1264:21: unexpected DefKind in ProjectionTy: AssocConst
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1609:9
stack backtrace:
stack backtrace:
   0:     0x7f5614d0a825 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h38611d6c58ff15d7
