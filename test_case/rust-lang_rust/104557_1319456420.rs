plain
...................iii.................................................................. 13816/13857
.........................................
failures:

---- [ui] src/test/ui/dyn-star/dyn-async-trait.rs stdout ----
error: test compilation failed although it shouldn't!
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dyn-star/dyn-async-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/dyn-async-trait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/dyn-async-trait/auxiliary" "--edition=2021"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_trait_selection/src/traits/select/confirmation.rs:1148:18: impossible case reached
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1555:9
stack backtrace:
   0:     0x7f1092e88bfe - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he21a45cd0a9fcfea
   0:     0x7f1092e88bfe - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he21a45cd0a9fcfea
   1:     0x7f1092ef8608 - core::fmt::write::h07bf6bedb8fe1686
   2:     0x7f1092e7aa61 - std::io::Write::write_fmt::he911249b0a85f65e
   3:     0x7f1092e88a01 - std::sys_common::backtrace::print::heb9f1050ca76dd88
   4:     0x7f1092e8bd64 - std::panicking::default_hook::{{closure}}::h7a33e1da318ac2df
   5:     0x7f1092e8ba29 - std::panicking::default_hook::h1ba3f9a3dc6fc44c
   6:     0x7f10938a99c4 - rustc_driver[a90893dd68686b9a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f1092e8c4b4 - std::panicking::rust_panic_with_hook::h8d13ad727f4c6f99
   8:     0x7f10964f2d63 - std[5045a5816feb3f0b]::panicking::begin_panic::<rustc_errors[1b15799f3752b02d]::ExplicitBug>::{closure#0}
   9:     0x7f10964f2be6 - std[5045a5816feb3f0b]::sys_common::backtrace::__rust_end_short_backtrace::<std[5045a5816feb3f0b]::panicking::begin_panic<rustc_errors[1b15799f3752b02d]::ExplicitBug>::{closure#0}, !>
  10:     0x7f109384c406 - std[5045a5816feb3f0b]::panicking::begin_panic::<rustc_errors[1b15799f3752b02d]::ExplicitBug>
  11:     0x7f10965c0576 - std[5045a5816feb3f0b]::panic::panic_any::<rustc_errors[1b15799f3752b02d]::ExplicitBug>
  12:     0x7f10965b2b07 - <rustc_errors[1b15799f3752b02d]::HandlerInner>::bug::<&alloc[b57ab99cad09ab94]::string::String>
  13:     0x7f10965aeb80 - <rustc_errors[1b15799f3752b02d]::Handler>::bug::<&alloc[b57ab99cad09ab94]::string::String>
  14:     0x7f10966771fe - rustc_middle[9c82130a49eafa81]::ty::context::tls::with_context_opt::<rustc_middle[9c82130a49eafa81]::ty::context::tls::with_opt<rustc_middle[9c82130a49eafa81]::util::bug::opt_span_bug_fmt<rustc_span[55dfc6b5854cff52]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  15:     0x7f1096678949 - rustc_middle[9c82130a49eafa81]::util::bug::opt_span_bug_fmt::<rustc_span[55dfc6b5854cff52]::span_encoding::Span>
  16:     0x7f1093854618 - rustc_middle[9c82130a49eafa81]::util::bug::bug_fmt
  17:     0x7f109624e3f4 - <rustc_trait_selection[dbb3d848c609818c]::traits::select::SelectionContext>::confirm_candidate
  18:     0x7f10962517fb - <rustc_trait_selection[dbb3d848c609818c]::traits::select::SelectionContext>::select
  19:     0x7f1093f6c721 - <rustc_hir_typeck[d0a88f4d206948dc]::coercion::Coerce>::coerce_unsized
  20:     0x7f1093f79cdf - <rustc_infer[48d8065074a90f9b]::infer::InferCtxt>::commit_if_ok::<rustc_infer[48d8065074a90f9b]::infer::InferOk<(alloc[b57ab99cad09ab94]::vec::Vec<rustc_middle[9c82130a49eafa81]::ty::adjustment::Adjustment>, rustc_middle[9c82130a49eafa81]::ty::Ty)>, rustc_middle[9c82130a49eafa81]::ty::error::TypeError, <rustc_hir_typeck[d0a88f4d206948dc]::coercion::Coerce>::coerce::{closure#0}>
  21:     0x7f1093f669d9 - <rustc_hir_typeck[d0a88f4d206948dc]::coercion::Coerce>::coerce
  22:     0x7f1093f796af - <rustc_infer[48d8065074a90f9b]::infer::InferCtxt>::commit_if_ok::<rustc_infer[48d8065074a90f9b]::infer::InferOk<(alloc[b57ab99cad09ab94]::vec::Vec<rustc_middle[9c82130a49eafa81]::ty::adjustment::Adjustment>, rustc_middle[9c82130a49eafa81]::ty::Ty)>, rustc_middle[9c82130a49eafa81]::ty::error::TypeError, <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::try_coerce::{closure#1}>
  23:     0x7f1093d9d0c1 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::try_coerce
  24:     0x7f1093dc81f9 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_argument_types
  25:     0x7f1093d9b61f - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::confirm_builtin_call
  26:     0x7f1093d98c5c - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_call
  27:     0x7f1093e1939f - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_kind
  28:     0x7f1093daf324 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  29:     0x7f1093e185d2 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  30:     0x7f1093dcfc06 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_block_with_expected
  31:     0x7f1093e1953b - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_kind
  32:     0x7f1093daf324 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  33:     0x7f1093e185d2 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  34:     0x7f1093e0bd23 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_match
  35:     0x7f1093e1936a - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_kind
  36:     0x7f1093daf324 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  37:     0x7f1093e185d2 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  38:     0x7f1093dcf4f2 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_stmt
  39:     0x7f1093dcfbc7 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_block_with_expected
  40:     0x7f1093e1b517 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_kind
  41:     0x7f1093daf324 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  42:     0x7f1093e185d2 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  43:     0x7f1093e0c76c - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_match
  44:     0x7f1093e1936a - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_kind
  45:     0x7f1093daf324 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  46:     0x7f1093e185d2 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  47:     0x7f1093dcfc06 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_block_with_expected
  48:     0x7f1093e1953b - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_kind
  49:     0x7f1093daf324 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  50:     0x7f1093e185d2 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  51:     0x7f1093e18e20 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_kind
  52:     0x7f1093daf324 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  53:     0x7f1093e185d2 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  54:     0x7f1093dcfc06 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_block_with_expected
  55:     0x7f1093e1953b - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_kind
  56:     0x7f1093daf324 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  57:     0x7f1093e185d2 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_with_expectation
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  58:     0x7f1093db0b5f - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_return_expr
  59:     0x7f1093f207cc - rustc_hir_typeck[d0a88f4d206948dc]::check::check_fn
  60:     0x7f1093e10ca8 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_closure
  61:     0x7f1093e18de2 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_kind
  62:     0x7f1093daf324 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  63:     0x7f1093e185d2 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  64:     0x7f1093dc8178 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_argument_types
  65:     0x7f1093d9b61f - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::confirm_builtin_call
  66:     0x7f1093d98c5c - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_call
  67:     0x7f1093e1939f - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_kind
  68:     0x7f1093daf324 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  69:     0x7f1093e185d2 - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  70:     0x7f1093db0b5f - <rustc_hir_typeck[d0a88f4d206948dc]::fn_ctxt::FnCtxt>::check_return_expr
  71:     0x7f1093f207cc - rustc_hir_typeck[d0a88f4d206948dc]::check::check_fn
  72:     0x7f1093eef254 - <rustc_hir_typeck[d0a88f4d206948dc]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[d0a88f4d206948dc]::typeck_with_fallback<rustc_hir_typeck[d0a88f4d206948dc]::typeck::{closure#0}>::{closure#0}::{closure#1}, &rustc_middle[9c82130a49eafa81]::ty::context::TypeckResults>
  73:     0x7f1093ed6283 - rustc_hir_typeck[d0a88f4d206948dc]::typeck
  74:     0x7f1095647469 - rustc_query_system[d8f2392ff58c36c4]::query::plumbing::try_execute_query::<rustc_query_impl[768b4e9ab4f38c38]::plumbing::QueryCtxt, rustc_query_system[d8f2392ff58c36c4]::query::caches::DefaultCache<rustc_span[55dfc6b5854cff52]::def_id::LocalDefId, &rustc_middle[9c82130a49eafa81]::ty::context::TypeckResults>>
  75:     0x7f1095775d4d - rustc_query_system[d8f2392ff58c36c4]::query::plumbing::get_query::<rustc_query_impl[768b4e9ab4f38c38]::queries::typeck, rustc_query_impl[768b4e9ab4f38c38]::plumbing::QueryCtxt>
  76:     0x7f10952dacd0 - <rustc_query_impl[768b4e9ab4f38c38]::Queries as rustc_middle[9c82130a49eafa81]::ty::query::QueryEngine>::typeck
  77:     0x7f109660ec05 - <rustc_middle[9c82130a49eafa81]::ty::context::TyCtxt>::typeck_opt_const_arg
  78:     0x7f1094a1b515 - rustc_mir_build[e8340dfb6c7f8ad6]::thir::cx::thir_body
  79:     0x7f1095779c41 - rustc_query_system[d8f2392ff58c36c4]::query::plumbing::get_query::<rustc_query_impl[768b4e9ab4f38c38]::queries::thir_body, rustc_query_impl[768b4e9ab4f38c38]::plumbing::QueryCtxt>
  80:     0x7f10952b84f6 - <rustc_query_impl[768b4e9ab4f38c38]::Queries as rustc_middle[9c82130a49eafa81]::ty::query::QueryEngine>::thir_body
  81:     0x7f1094999fce - rustc_mir_build[e8340dfb6c7f8ad6]::build::mir_built
  82:     0x7f1095634ba3 - rustc_query_system[d8f2392ff58c36c4]::query::plumbing::try_execute_query::<rustc_query_impl[768b4e9ab4f38c38]::plumbing::QueryCtxt, rustc_query_system[d8f2392ff58c36c4]::query::caches::DefaultCache<rustc_middle[9c82130a49eafa81]::ty::WithOptConstParam<rustc_span[55dfc6b5854cff52]::def_id::LocalDefId>, &rustc_data_structures[c0820871839be3d5]::steal::Steal<rustc_middle[9c82130a49eafa81]::mir::Body>>>
  83:     0x7f10957780dc - rustc_query_system[d8f2392ff58c36c4]::query::plumbing::get_query::<rustc_query_impl[768b4e9ab4f38c38]::queries::mir_built, rustc_query_impl[768b4e9ab4f38c38]::plumbing::QueryCtxt>
  84:     0x7f10952baa54 - <rustc_query_impl[768b4e9ab4f38c38]::Queries as rustc_middle[9c82130a49eafa81]::ty::query::QueryEngine>::mir_built
  85:     0x7f10944d047c - rustc_mir_transform[28fc2a838fedb408]::check_unsafety::unsafety_check_result
  86:     0x7f10944cd56e - <rustc_mir_transform[28fc2a838fedb408]::check_unsafety::provide::{closure#0} as core[22ea3c23ced562b2]::ops::function::FnOnce<(rustc_middle[9c82130a49eafa81]::ty::context::TyCtxt, rustc_span[55dfc6b5854cff52]::def_id::LocalDefId)>>::call_once
  87:     0x7f1095648fc9 - rustc_query_system[d8f2392ff58c36c4]::query::plumbing::try_execute_query::<rustc_query_impl[768b4e9ab4f38c38]::plumbing::QueryCtxt, rustc_query_system[d8f2392ff58c36c4]::query::caches::DefaultCache<rustc_span[55dfc6b5854cff52]::def_id::LocalDefId, &rustc_middle[9c82130a49eafa81]::mir::query::UnsafetyCheckResult>>
  88:     0x7f1095747ead - rustc_query_system[d8f2392ff58c36c4]::query::plumbing::get_query::<rustc_query_impl[768b4e9ab4f38c38]::queries::unsafety_check_result, rustc_query_impl[768b4e9ab4f38c38]::plumbing::QueryCtxt>
  89:     0x7f10952d02b0 - <rustc_query_impl[768b4e9ab4f38c38]::Queries as rustc_middle[9c82130a49eafa81]::ty::query::QueryEngine>::unsafety_check_result
  90:     0x7f10944218d3 - rustc_mir_transform[28fc2a838fedb408]::mir_const
  91:     0x7f1095634ba3 - rustc_query_system[d8f2392ff58c36c4]::query::plumbing::try_execute_query::<rustc_query_impl[768b4e9ab4f38c38]::plumbing::QueryCtxt, rustc_query_system[d8f2392ff58c36c4]::query::caches::DefaultCache<rustc_middle[9c82130a49eafa81]::ty::WithOptConstParam<rustc_span[55dfc6b5854cff52]::def_id::LocalDefId>, &rustc_data_structures[c0820871839be3d5]::steal::Steal<rustc_middle[9c82130a49eafa81]::mir::Body>>>
  92:     0x7f10957781ff - rustc_query_system[d8f2392ff58c36c4]::query::plumbing::get_query::<rustc_query_impl[768b4e9ab4f38c38]::queries::mir_const, rustc_query_impl[768b4e9ab4f38c38]::plumbing::QueryCtxt>
  93:     0x7f10952bb1c4 - <rustc_query_impl[768b4e9ab4f38c38]::Queries as rustc_middle[9c82130a49eafa81]::ty::query::QueryEngine>::mir_const
  94:     0x7f1094422244 - rustc_mir_transform[28fc2a838fedb408]::mir_promoted
  95:     0x7f1095724a3f - rustc_query_system[d8f2392ff58c36c4]::query::plumbing::get_query::<rustc_query_impl[768b4e9ab4f38c38]::queries::mir_promoted, rustc_query_impl[768b4e9ab4f38c38]::plumbing::QueryCtxt>
  96:     0x7f10952be664 - <rustc_query_impl[768b4e9ab4f38c38]::Queries as rustc_middle[9c82130a49eafa81]::ty::query::QueryEngine>::mir_promoted
  97:     0x7f1094c6cdf8 - rustc_borrowck[900b2383ea526a4d]::mir_borrowck
  98:     0x7f1094c39e6e - <rustc_borrowck[900b2383ea526a4d]::provide::{closure#0} as core[22ea3c23ced562b2]::ops::function::FnOnce<(rustc_middle[9c82130a49eafa81]::ty::context::TyCtxt, rustc_span[55dfc6b5854cff52]::def_id::LocalDefId)>>::call_once
  99:     0x7f1095648219 - rustc_query_system[d8f2392ff58c36c4]::query::plumbing::try_execute_query::<rustc_query_impl[768b4e9ab4f38c38]::plumbing::QueryCtxt, rustc_query_system[d8f2392ff58c36c4]::query::caches::DefaultCache<rustc_span[55dfc6b5854cff52]::def_id::LocalDefId, &rustc_middle[9c82130a49eafa81]::mir::query::BorrowCheckResult>>
 100:     0x7f1095724345 - rustc_query_system[d8f2392ff58c36c4]::query::plumbing::get_query::<rustc_query_impl[768b4e9ab4f38c38]::queries::mir_borrowck, rustc_query_impl[768b4e9ab4f38c38]::plumbing::QueryCtxt>
 101:     0x7f10952dd920 - <rustc_query_impl[768b4e9ab4f38c38]::Queries as rustc_middle[9c82130a49eafa81]::ty::query::QueryEngine>::mir_borrowck
 102:     0x7f10940d1c7e - rustc_hir_analysis[23634df4af05b48e]::collect::type_of::find_opaque_ty_constraints_for_rpit
 103:     0x7f10940d0eb4 - rustc_hir_analysis[23634df4af05b48e]::collect::type_of::type_of
 104:     0x7f1095663ec0 - rustc_query_system[d8f2392ff58c36c4]::query::plumbing::try_execute_query::<rustc_query_impl[768b4e9ab4f38c38]::plumbing::QueryCtxt, rustc_query_system[d8f2392ff58c36c4]::query::caches::DefaultCache<rustc_span[55dfc6b5854cff52]::def_id::DefId, rustc_middle[9c82130a49eafa81]::ty::Ty>>
 105:     0x7f1095775fe7 - rustc_query_system[d8f2392ff58c36c4]::query::plumbing::get_query::<rustc_query_impl[768b4e9ab4f38c38]::queries::type_of, rustc_query_impl[768b4e9ab4f38c38]::plumbing::QueryCtxt>
 106:     0x7f10952b06b5 - <rustc_query_impl[768b4e9ab4f38c38]::Queries as rustc_middle[9c82130a49eafa81]::ty::query::QueryEngine>::type_of
 107:     0x7f1094064533 - rustc_hir_analysis[23634df4af05b48e]::check::check::check_opaque
 108:     0x7f1094068078 - rustc_hir_analysis[23634df4af05b48e]::check::check::check_item_type
 109:     0x7f10940732fa - rustc_hir_analysis[23634df4af05b48e]::check::check::check_mod_item_types
 110:     0x7f109564ab52 - rustc_query_system[d8f2392ff58c36c4]::query::plumbing::try_execute_query::<rustc_query_impl[768b4e9ab4f38c38]::plumbing::QueryCtxt, rustc_query_system[d8f2392ff58c36c4]::query::caches::DefaultCache<rustc_span[55dfc6b5854cff52]::def_id::LocalDefId, ()>>
 111:     0x7f10957420aa - rustc_query_system[d8f2392ff58c36c4]::query::plumbing::get_query::<rustc_query_impl[768b4e9ab4f38c38]::queries::check_mod_item_types, rustc_query_impl[768b4e9ab4f38c38]::plumbing::QueryCtxt>
 112:     0x7f10952d6310 - <rustc_query_impl[768b4e9ab4f38c38]::Queries as rustc_middle[9c82130a49eafa81]::ty::query::QueryEngine>::check_mod_item_types
 113:     0x7f1094021059 - <rustc_middle[9c82130a49eafa81]::hir::map::Map>::for_each_module::<rustc_hir_analysis[23634df4af05b48e]::check_crate::{closure#6}::{closure#0}>
 114:     0x7f10940a16a2 - <rustc_session[b67b69a0ad389d4d]::session::Session>::time::<(), rustc_hir_analysis[23634df4af05b48e]::check_crate::{closure#6}>
 115:     0x7f109423a7b1 - rustc_hir_analysis[23634df4af05b48e]::check_crate
 116:     0x7f10939ff321 - rustc_interface[babf6d8592ed0a4f]::passes::analysis
 117:     0x7f109568b95f - rustc_query_system[d8f2392ff58c36c4]::query::plumbing::try_execute_query::<rustc_query_impl[768b4e9ab4f38c38]::plumbing::QueryCtxt, rustc_query_system[d8f2392ff58c36c4]::query::caches::DefaultCache<(), core[22ea3c23ced562b2]::result::Result<(), rustc_errors[1b15799f3752b02d]::ErrorGuaranteed>>>
 118:     0x7f10957760fb - rustc_query_system[d8f2392ff58c36c4]::query::plumbing::get_query::<rustc_query_impl[768b4e9ab4f38c38]::queries::analysis, rustc_query_impl[768b4e9ab4f38c38]::plumbing::QueryCtxt>
 119:     0x7f10952b159a - <rustc_query_impl[768b4e9ab4f38c38]::Queries as rustc_middle[9c82130a49eafa81]::ty::query::QueryEngine>::analysis
 120:     0x7f10939044d9 - <rustc_interface[babf6d8592ed0a4f]::passes::QueryContext>::enter::<rustc_driver[a90893dd68686b9a]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[22ea3c23ced562b2]::result::Result<(), rustc_errors[1b15799f3752b02d]::ErrorGuaranteed>>
 121:     0x7f109391941a - <rustc_interface[babf6d8592ed0a4f]::interface::Compiler>::enter::<rustc_driver[a90893dd68686b9a]::run_compiler::{closure#1}::{closure#2}, core[22ea3c23ced562b2]::result::Result<core[22ea3c23ced562b2]::option::Option<rustc_interface[babf6d8592ed0a4f]::queries::Linker>, rustc_errors[1b15799f3752b02d]::ErrorGuaranteed>>
 122:     0x7f109389709e - rustc_span[55dfc6b5854cff52]::with_source_map::<core[22ea3c23ced562b2]::result::Result<(), rustc_errors[1b15799f3752b02d]::ErrorGuaranteed>, rustc_interface[babf6d8592ed0a4f]::interface::run_compiler<core[22ea3c23ced562b2]::result::Result<(), rustc_errors[1b15799f3752b02d]::ErrorGuaranteed>, rustc_driver[a90893dd68686b9a]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
 123:     0x7f109390be5c - <scoped_tls[88f4a1de410ddfe1]::ScopedKey<rustc_span[55dfc6b5854cff52]::SessionGlobals>>::set::<rustc_interface[babf6d8592ed0a4f]::interface::run_compiler<core[22ea3c23ced562b2]::result::Result<(), rustc_errors[1b15799f3752b02d]::ErrorGuaranteed>, rustc_driver[a90893dd68686b9a]::run_compiler::{closure#1}>::{closure#0}, core[22ea3c23ced562b2]::result::Result<(), rustc_errors[1b15799f3752b02d]::ErrorGuaranteed>>
 124:     0x7f1093907aa9 - std[5045a5816feb3f0b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[babf6d8592ed0a4f]::util::run_in_thread_pool_with_globals<rustc_interface[babf6d8592ed0a4f]::interface::run_compiler<core[22ea3c23ced562b2]::result::Result<(), rustc_errors[1b15799f3752b02d]::ErrorGuaranteed>, rustc_driver[a90893dd68686b9a]::run_compiler::{closure#1}>::{closure#0}, core[22ea3c23ced562b2]::result::Result<(), rustc_errors[1b15799f3752b02d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[22ea3c23ced562b2]::result::Result<(), rustc_errors[1b15799f3752b02d]::ErrorGuaranteed>>
 125:     0x7f1093910c96 - std[5045a5816feb3f0b]::panicking::try::<core[22ea3c23ced562b2]::result::Result<(), rustc_errors[1b15799f3752b02d]::ErrorGuaranteed>, core[22ea3c23ced562b2]::panic::unwind_safe::AssertUnwindSafe<<std[5045a5816feb3f0b]::thread::Builder>::spawn_unchecked_<rustc_interface[babf6d8592ed0a4f]::util::run_in_thread_pool_with_globals<rustc_interface[babf6d8592ed0a4f]::interface::run_compiler<core[22ea3c23ced562b2]::result::Result<(), rustc_errors[1b15799f3752b02d]::ErrorGuaranteed>, rustc_driver[a90893dd68686b9a]::run_compiler::{closure#1}>::{closure#0}, core[22ea3c23ced562b2]::result::Result<(), rustc_errors[1b15799f3752b02d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[22ea3c23ced562b2]::result::Result<(), rustc_errors[1b15799f3752b02d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
 126:     0x7f10938b8f3a - <<std[5045a5816feb3f0b]::thread::Builder>::spawn_unchecked_<rustc_interface[babf6d8592ed0a4f]::util::run_in_thread_pool_with_globals<rustc_interface[babf6d8592ed0a4f]::interface::run_compiler<core[22ea3c23ced562b2]::result::Result<(), rustc_errors[1b15799f3752b02d]::ErrorGuaranteed>, rustc_driver[a90893dd68686b9a]::run_compiler::{closure#1}>::{closure#0}, core[22ea3c23ced562b2]::result::Result<(), rustc_errors[1b15799f3752b02d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[22ea3c23ced562b2]::result::Result<(), rustc_errors[1b15799f3752b02d]::ErrorGuaranteed>>::{closure#1} as core[22ea3c23ced562b2]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
 127:     0x7f1092e98bde - std::sys::unix::thread::Thread::new::thread_start::h325beb26314973d3
 128:     0x7f1092c2eb43 - <unknown>
 129:     0x7f1092cc0a00 - <unknown>
 130:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.67.0-nightly (eb78ff6a9 2022-11-17) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `do_counter`
#1 [thir_body] building THIR for `do_counter`
#2 [mir_built] building MIR for `do_counter`
#3 [unsafety_check_result] unsafety-checking `do_counter`
#4 [mir_const] preparing `do_counter` for borrow checking
#5 [mir_promoted] processing MIR for `do_counter`
#6 [mir_borrowck] borrow-checking `do_counter`
#7 [type_of] computing type of `do_counter::{opaque#0}`
#8 [check_mod_item_types] checking item types in top-level module
#9 [analysis] running analysis passes on this crate
error: aborting due to previous error
------------------------------------------


