
error: internal compiler error: compiler/rustc_middle/src/ty/subst.rs:703:13: Region parameter out of range when substituting in region 'a (index=1, substs = ['_#21r])

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/70f8737b2f5d3bf7d6b784fad00b663b7ff9feda/compiler/rustc_errors/src/lib.rs:1560:9
stack backtrace:
   0:     0x7fcdbf80a0da - std::backtrace_rs::backtrace::libunwind::trace::h06e8f32b98d440be
                               at /rustc/70f8737b2f5d3bf7d6b784fad00b663b7ff9feda/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fcdbf80a0da - std::backtrace_rs::backtrace::trace_unsynchronized::h8a56795adafc3159
                               at /rustc/70f8737b2f5d3bf7d6b784fad00b663b7ff9feda/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fcdbf80a0da - std::sys_common::backtrace::_print_fmt::h7ba19acb3601abb5
                               at /rustc/70f8737b2f5d3bf7d6b784fad00b663b7ff9feda/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7fcdbf80a0da - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha37df576307c1a49
                               at /rustc/70f8737b2f5d3bf7d6b784fad00b663b7ff9feda/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7fcdbb7c8bae - core::fmt::write::hd3f2e7e755b45b01
                               at /rustc/70f8737b2f5d3bf7d6b784fad00b663b7ff9feda/library/core/src/fmt/mod.rs:1208:17
   5:     0x7fcdbf7fe3a5 - std::io::Write::write_fmt::hc1388e059aac0a41
                               at /rustc/70f8737b2f5d3bf7d6b784fad00b663b7ff9feda/library/std/src/io/mod.rs:1682:15
   6:     0x7fcdbf809ea5 - std::sys_common::backtrace::_print::h429db1082d921ab8
                               at /rustc/70f8737b2f5d3bf7d6b784fad00b663b7ff9feda/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7fcdbf809ea5 - std::sys_common::backtrace::print::h8d611e0005d3ccb9
                               at /rustc/70f8737b2f5d3bf7d6b784fad00b663b7ff9feda/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7fcdbf80c20f - std::panicking::default_hook::{{closure}}::h07380c607f829a79
                               at /rustc/70f8737b2f5d3bf7d6b784fad00b663b7ff9feda/library/std/src/panicking.rs:267:22
   9:     0x7fcdbf80bf4b - std::panicking::default_hook::h78998b3f3926502a
                               at /rustc/70f8737b2f5d3bf7d6b784fad00b663b7ff9feda/library/std/src/panicking.rs:286:9
  10:     0x7fcdbea64284 - <rustc_driver[153e38e44d5410f9]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[1d8afb50f4e7a45]::ops::function::FnOnce<(&core[1d8afb50f4e7a45]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
  11:     0x7fcdbf80ca0d - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hbe7c77b7cbfe1f21
                               at /rustc/70f8737b2f5d3bf7d6b784fad00b663b7ff9feda/library/alloc/src/boxed.rs:2032:9
  12:     0x7fcdbf80ca0d - std::panicking::rust_panic_with_hook::hda16cab0ff009a11
                               at /rustc/70f8737b2f5d3bf7d6b784fad00b663b7ff9feda/library/std/src/panicking.rs:692:13
  13:     0x7fcdbeeb3841 - std[2b5db76a972e300f]::panicking::begin_panic::<rustc_errors[6e875e03adfb8e66]::ExplicitBug>::{closure#0}
  14:     0x7fcdbeeaff46 - std[2b5db76a972e300f]::sys_common::backtrace::__rust_end_short_backtrace::<std[2b5db76a972e300f]::panicking::begin_panic<rustc_errors[6e875e03adfb8e66]::ExplicitBug>::{closure#0}, !>
  15:     0x7fcdbef01416 - std[2b5db76a972e300f]::panicking::begin_panic::<rustc_errors[6e875e03adfb8e66]::ExplicitBug>
  16:     0x7fcdbeead9a6 - std[2b5db76a972e300f]::panic::panic_any::<rustc_errors[6e875e03adfb8e66]::ExplicitBug>
  17:     0x7fcdbeead8d6 - <rustc_errors[6e875e03adfb8e66]::HandlerInner>::bug::<&alloc[eb468f42ab80a7c0]::string::String>
  18:     0x7fcdbeead540 - <rustc_errors[6e875e03adfb8e66]::Handler>::bug::<&alloc[eb468f42ab80a7c0]::string::String>
  19:     0x7fcdbef609ae - rustc_middle[62e623a39719183a]::ty::context::tls::with_context_opt::<rustc_middle[62e623a39719183a]::ty::context::tls::with_opt<rustc_middle[62e623a39719183a]::util::bug::opt_span_bug_fmt<rustc_span[cdf434c68ce75247]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  20:     0x7fcdbef60b76 - rustc_middle[62e623a39719183a]::util::bug::opt_span_bug_fmt::<rustc_span[cdf434c68ce75247]::span_encoding::Span>
  21:     0x7fcdbcd1f473 - rustc_middle[62e623a39719183a]::util::bug::bug_fmt
  22:     0x7fcdbeebff31 - <rustc_middle[62e623a39719183a]::ty::subst::SubstFolder as rustc_middle[62e623a39719183a]::ty::fold::TypeFolder>::fold_region::region_param_out_of_range
  23:     0x7fcdbca6f3e0 - <rustc_middle[62e623a39719183a]::ty::generics::GenericPredicates>::instantiate_into
  24:     0x7fcdbccf95dd - <rustc_trait_selection[a1154a3527114d9c]::traits::wf::WfPredicates>::nominal_obligations_inner
  25:     0x7fcdbccf7990 - <rustc_trait_selection[a1154a3527114d9c]::traits::wf::WfPredicates>::compute
  26:     0x7fcdbcce6587 - <rustc_trait_selection[a1154a3527114d9c]::traits::fulfill::FulfillProcessor as rustc_data_structures[9b000dd414eff99d]::obligation_forest::ObligationProcessor>::process_obligation
  27:     0x7fcdbcce385f - <rustc_data_structures[9b000dd414eff99d]::obligation_forest::ObligationForest<rustc_trait_selection[a1154a3527114d9c]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[a1154a3527114d9c]::traits::fulfill::FulfillProcessor>
  28:     0x7fcdbcce3318 - <rustc_trait_selection[a1154a3527114d9c]::traits::fulfill::FulfillmentContext as rustc_infer[d41ecc1920ef9403]::traits::engine::TraitEngine>::select_where_possible
  29:     0x7fcdbdb36c8d - <rustc_hir_typeck[86d34420e98aa3da]::fn_ctxt::FnCtxt>::try_find_coercion_lub::<rustc_hir[333af84aa7ad67db]::hir::Arm>
  30:     0x7fcdbccb4de8 - <rustc_hir_typeck[86d34420e98aa3da]::fn_ctxt::FnCtxt>::check_match::{closure#0}
  31:     0x7fcdbcc51e17 - <rustc_hir_typeck[86d34420e98aa3da]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  32:     0x7fcdbcc96dce - <rustc_hir_typeck[86d34420e98aa3da]::fn_ctxt::FnCtxt>::check_block_with_expected
  33:     0x7fcdbcc544f3 - <rustc_hir_typeck[86d34420e98aa3da]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  34:     0x7fcdbccb495e - <rustc_hir_typeck[86d34420e98aa3da]::fn_ctxt::FnCtxt>::check_match::{closure#0}
  35:     0x7fcdbcc51e17 - <rustc_hir_typeck[86d34420e98aa3da]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  36:     0x7fcdbcc99dba - <rustc_hir_typeck[86d34420e98aa3da]::fn_ctxt::FnCtxt>::check_decl
  37:     0x7fcdbcc547bc - <rustc_hir_typeck[86d34420e98aa3da]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  38:     0x7fcdbcc51f6b - <rustc_hir_typeck[86d34420e98aa3da]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  39:     0x7fcdbcc96dce - <rustc_hir_typeck[86d34420e98aa3da]::fn_ctxt::FnCtxt>::check_block_with_expected
  40:     0x7fcdbcc4ffe4 - <rustc_hir_typeck[86d34420e98aa3da]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  41:     0x7fcdbccb495e - <rustc_hir_typeck[86d34420e98aa3da]::fn_ctxt::FnCtxt>::check_match::{closure#0}
  42:     0x7fcdbcc51e17 - <rustc_hir_typeck[86d34420e98aa3da]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  43:     0x7fcdbcc96ecb - <rustc_hir_typeck[86d34420e98aa3da]::fn_ctxt::FnCtxt>::check_block_with_expected
  44:     0x7fcdbcc4ffe4 - <rustc_hir_typeck[86d34420e98aa3da]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  45:     0x7fcdbcc52303 - <rustc_hir_typeck[86d34420e98aa3da]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  46:     0x7fcdbcc96ecb - <rustc_hir_typeck[86d34420e98aa3da]::fn_ctxt::FnCtxt>::check_block_with_expected
  47:     0x7fcdbcc4ffe4 - <rustc_hir_typeck[86d34420e98aa3da]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  48:     0x7fcdbcef85b3 - <rustc_hir_typeck[86d34420e98aa3da]::fn_ctxt::FnCtxt>::check_return_expr
  49:     0x7fcdbcef0004 - rustc_hir_typeck[86d34420e98aa3da]::check::check_fn
  50:     0x7fcdbcd43691 - <rustc_hir_typeck[86d34420e98aa3da]::fn_ctxt::FnCtxt>::check_expr_closure
  51:     0x7fcdbcc53d3b - <rustc_hir_typeck[86d34420e98aa3da]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  52:     0x7fcdbccc5933 - <rustc_hir_typeck[86d34420e98aa3da]::fn_ctxt::FnCtxt>::check_argument_types
  53:     0x7fcdbdde0a50 - <rustc_hir_typeck[86d34420e98aa3da]::fn_ctxt::FnCtxt>::check_call
  54:     0x7fcdbcc4f896 - <rustc_hir_typeck[86d34420e98aa3da]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  55:     0x7fcdbcef85b3 - <rustc_hir_typeck[86d34420e98aa3da]::fn_ctxt::FnCtxt>::check_return_expr
  56:     0x7fcdbcef0004 - rustc_hir_typeck[86d34420e98aa3da]::check::check_fn
  57:     0x7fcdbcedac03 - <rustc_hir_typeck[86d34420e98aa3da]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[86d34420e98aa3da]::typeck_with_fallback<rustc_hir_typeck[86d34420e98aa3da]::typeck::{closure#0}>::{closure#0}::{closure#1}, &rustc_middle[62e623a39719183a]::ty::context::TypeckResults>
  58:     0x7fcdbcecc2f1 - rustc_hir_typeck[86d34420e98aa3da]::typeck
  59:     0x7fcdbced705c - <rustc_query_system[ef279d85abd3e98b]::dep_graph::graph::DepGraph<rustc_middle[62e623a39719183a]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[62e623a39719183a]::ty::context::TyCtxt, rustc_span[cdf434c68ce75247]::def_id::LocalDefId, &rustc_middle[62e623a39719183a]::ty::context::TypeckResults>
  60:     0x7fcdbcecaac3 - rustc_query_system[ef279d85abd3e98b]::query::plumbing::try_execute_query::<rustc_query_impl[934f89a4ce3db597]::plumbing::QueryCtxt, rustc_query_system[ef279d85abd3e98b]::query::caches::DefaultCache<rustc_span[cdf434c68ce75247]::def_id::LocalDefId, &rustc_middle[62e623a39719183a]::ty::context::TypeckResults>>
  61:     0x7fcdbe431497 - <rustc_query_impl[934f89a4ce3db597]::Queries as rustc_middle[62e623a39719183a]::ty::query::QueryEngine>::typeck
  62:     0x7fcdbd334c15 - rustc_mir_build[c8a67276e5040a53]::thir::cx::thir_body
  63:     0x7fcdbd97ea5f - <rustc_query_system[ef279d85abd3e98b]::dep_graph::graph::DepGraph<rustc_middle[62e623a39719183a]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[62e623a39719183a]::ty::context::TyCtxt, rustc_middle[62e623a39719183a]::ty::WithOptConstParam<rustc_span[cdf434c68ce75247]::def_id::LocalDefId>, core[1d8afb50f4e7a45]::result::Result<(&rustc_data_structures[9b000dd414eff99d]::steal::Steal<rustc_middle[62e623a39719183a]::thir::Thir>, rustc_middle[62e623a39719183a]::thir::ExprId), rustc_errors[6e875e03adfb8e66]::ErrorGuaranteed>>
  64:     0x7fcdbd97656b - rustc_query_system[ef279d85abd3e98b]::query::plumbing::try_execute_query::<rustc_query_impl[934f89a4ce3db597]::plumbing::QueryCtxt, rustc_query_system[ef279d85abd3e98b]::query::caches::DefaultCache<rustc_middle[62e623a39719183a]::ty::WithOptConstParam<rustc_span[cdf434c68ce75247]::def_id::LocalDefId>, core[1d8afb50f4e7a45]::result::Result<(&rustc_data_structures[9b000dd414eff99d]::steal::Steal<rustc_middle[62e623a39719183a]::thir::Thir>, rustc_middle[62e623a39719183a]::thir::ExprId), rustc_errors[6e875e03adfb8e66]::ErrorGuaranteed>>>
  65:     0x7fcdbd96eed7 - rustc_mir_build[c8a67276e5040a53]::build::mir_built
  66:     0x7fcdbd8bd6ae - <rustc_query_system[ef279d85abd3e98b]::dep_graph::graph::DepGraph<rustc_middle[62e623a39719183a]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[62e623a39719183a]::ty::context::TyCtxt, rustc_middle[62e623a39719183a]::ty::WithOptConstParam<rustc_span[cdf434c68ce75247]::def_id::LocalDefId>, &rustc_data_structures[9b000dd414eff99d]::steal::Steal<rustc_middle[62e623a39719183a]::mir::Body>>
  67:     0x7fcdbd8bc2c4 - rustc_query_system[ef279d85abd3e98b]::query::plumbing::try_execute_query::<rustc_query_impl[934f89a4ce3db597]::plumbing::QueryCtxt, rustc_query_system[ef279d85abd3e98b]::query::caches::DefaultCache<rustc_middle[62e623a39719183a]::ty::WithOptConstParam<rustc_span[cdf434c68ce75247]::def_id::LocalDefId>, &rustc_data_structures[9b000dd414eff99d]::steal::Steal<rustc_middle[62e623a39719183a]::mir::Body>>>
  68:     0x7fcdbd8c8355 - rustc_mir_transform[ec92a334231d7727]::check_unsafety::unsafety_check_result
  69:     0x7fcdbd8c2b88 - <rustc_mir_transform[ec92a334231d7727]::check_unsafety::provide::{closure#0} as core[1d8afb50f4e7a45]::ops::function::FnOnce<(rustc_middle[62e623a39719183a]::ty::context::TyCtxt, rustc_span[cdf434c68ce75247]::def_id::LocalDefId)>>::call_once
  70:     0x7fcdbd8c7bbc - <rustc_query_system[ef279d85abd3e98b]::dep_graph::graph::DepGraph<rustc_middle[62e623a39719183a]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[62e623a39719183a]::ty::context::TyCtxt, rustc_span[cdf434c68ce75247]::def_id::LocalDefId, &rustc_middle[62e623a39719183a]::mir::query::UnsafetyCheckResult>
  71:     0x7fcdbd8c20e3 - rustc_query_system[ef279d85abd3e98b]::query::plumbing::try_execute_query::<rustc_query_impl[934f89a4ce3db597]::plumbing::QueryCtxt, rustc_query_system[ef279d85abd3e98b]::query::caches::DefaultCache<rustc_span[cdf434c68ce75247]::def_id::LocalDefId, &rustc_middle[62e623a39719183a]::mir::query::UnsafetyCheckResult>>
  72:     0x7fcdbd8bf2d5 - rustc_mir_transform[ec92a334231d7727]::mir_const
  73:     0x7fcdbd8bd6ae - <rustc_query_system[ef279d85abd3e98b]::dep_graph::graph::DepGraph<rustc_middle[62e623a39719183a]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[62e623a39719183a]::ty::context::TyCtxt, rustc_middle[62e623a39719183a]::ty::WithOptConstParam<rustc_span[cdf434c68ce75247]::def_id::LocalDefId>, &rustc_data_structures[9b000dd414eff99d]::steal::Steal<rustc_middle[62e623a39719183a]::mir::Body>>
  74:     0x7fcdbd8bc2c4 - rustc_query_system[ef279d85abd3e98b]::query::plumbing::try_execute_query::<rustc_query_impl[934f89a4ce3db597]::plumbing::QueryCtxt, rustc_query_system[ef279d85abd3e98b]::query::caches::DefaultCache<rustc_middle[62e623a39719183a]::ty::WithOptConstParam<rustc_span[cdf434c68ce75247]::def_id::LocalDefId>, &rustc_data_structures[9b000dd414eff99d]::steal::Steal<rustc_middle[62e623a39719183a]::mir::Body>>>
  75:     0x7fcdbd9c4d1c - rustc_mir_transform[ec92a334231d7727]::mir_promoted
  76:     0x7fcdbd9c438f - <rustc_query_system[ef279d85abd3e98b]::dep_graph::graph::DepGraph<rustc_middle[62e623a39719183a]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[62e623a39719183a]::ty::context::TyCtxt, rustc_middle[62e623a39719183a]::ty::WithOptConstParam<rustc_span[cdf434c68ce75247]::def_id::LocalDefId>, (&rustc_data_structures[9b000dd414eff99d]::steal::Steal<rustc_middle[62e623a39719183a]::mir::Body>, &rustc_data_structures[9b000dd414eff99d]::steal::Steal<rustc_index[c0756d88214f2801]::vec::IndexVec<rustc_middle[62e623a39719183a]::mir::Promoted, rustc_middle[62e623a39719183a]::mir::Body>>)>
  77:     0x7fcdbd9c30bf - rustc_query_system[ef279d85abd3e98b]::query::plumbing::try_execute_query::<rustc_query_impl[934f89a4ce3db597]::plumbing::QueryCtxt, rustc_query_system[ef279d85abd3e98b]::query::caches::DefaultCache<rustc_middle[62e623a39719183a]::ty::WithOptConstParam<rustc_span[cdf434c68ce75247]::def_id::LocalDefId>, (&rustc_data_structures[9b000dd414eff99d]::steal::Steal<rustc_middle[62e623a39719183a]::mir::Body>, &rustc_data_structures[9b000dd414eff99d]::steal::Steal<rustc_index[c0756d88214f2801]::vec::IndexVec<rustc_middle[62e623a39719183a]::mir::Promoted, rustc_middle[62e623a39719183a]::mir::Body>>)>>
  78:     0x7fcdbd9c10e1 - rustc_borrowck[4aae648b9819d881]::mir_borrowck
  79:     0x7fcdbd9c0aa1 - <rustc_borrowck[4aae648b9819d881]::provide::{closure#0} as core[1d8afb50f4e7a45]::ops::function::FnOnce<(rustc_middle[62e623a39719183a]::ty::context::TyCtxt, rustc_span[cdf434c68ce75247]::def_id::LocalDefId)>>::call_once
  80:     0x7fcdbd961ffc - <rustc_query_system[ef279d85abd3e98b]::dep_graph::graph::DepGraph<rustc_middle[62e623a39719183a]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[62e623a39719183a]::ty::context::TyCtxt, rustc_span[cdf434c68ce75247]::def_id::LocalDefId, &rustc_middle[62e623a39719183a]::mir::query::BorrowCheckResult>
  81:     0x7fcdbd960ea5 - rustc_query_system[ef279d85abd3e98b]::query::plumbing::try_execute_query::<rustc_query_impl[934f89a4ce3db597]::plumbing::QueryCtxt, rustc_query_system[ef279d85abd3e98b]::query::caches::DefaultCache<rustc_span[cdf434c68ce75247]::def_id::LocalDefId, &rustc_middle[62e623a39719183a]::mir::query::BorrowCheckResult>>
  82:     0x7fcdbe431843 - <rustc_query_impl[934f89a4ce3db597]::Queries as rustc_middle[62e623a39719183a]::ty::query::QueryEngine>::mir_borrowck
  83:     0x7fcdbe28fb1e - rustc_hir_analysis[f648eabd16f2da8d]::collect::type_of::type_of
  84:     0x7fcdbd2f8123 - <rustc_query_system[ef279d85abd3e98b]::dep_graph::graph::DepGraph<rustc_middle[62e623a39719183a]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[62e623a39719183a]::ty::context::TyCtxt, rustc_span[cdf434c68ce75247]::def_id::DefId, rustc_middle[62e623a39719183a]::ty::Ty>
  85:     0x7fcdbd2f5fe6 - rustc_query_system[ef279d85abd3e98b]::query::plumbing::get_query::<rustc_query_impl[934f89a4ce3db597]::queries::type_of, rustc_query_impl[934f89a4ce3db597]::plumbing::QueryCtxt>
  86:     0x7fcdbcadae27 - rustc_hir_analysis[f648eabd16f2da8d]::check::check::check_mod_item_types
  87:     0x7fcdbd307ecc - <rustc_query_system[ef279d85abd3e98b]::dep_graph::graph::DepGraph<rustc_middle[62e623a39719183a]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[62e623a39719183a]::ty::context::TyCtxt, rustc_span[cdf434c68ce75247]::def_id::LocalDefId, ()>
  88:     0x7fcdbd305f2d - rustc_query_system[ef279d85abd3e98b]::query::plumbing::try_execute_query::<rustc_query_impl[934f89a4ce3db597]::plumbing::QueryCtxt, rustc_query_system[ef279d85abd3e98b]::query::caches::DefaultCache<rustc_span[cdf434c68ce75247]::def_id::LocalDefId, ()>>
  89:     0x7fcdbde8aa0e - rustc_query_system[ef279d85abd3e98b]::query::plumbing::get_query::<rustc_query_impl[934f89a4ce3db597]::queries::check_mod_item_types, rustc_query_impl[934f89a4ce3db597]::plumbing::QueryCtxt>
  90:     0x7fcdbe150fec - <rustc_middle[62e623a39719183a]::hir::map::Map>::for_each_module::<rustc_hir_analysis[f648eabd16f2da8d]::check_crate::{closure#6}::{closure#0}>
  91:     0x7fcdbce5fc32 - rustc_hir_analysis[f648eabd16f2da8d]::check_crate
  92:     0x7fcdbce5f87b - rustc_interface[d2ac3f3b3bfa4614]::passes::analysis
  93:     0x7fcdbe2534ef - <rustc_query_system[ef279d85abd3e98b]::dep_graph::graph::DepGraph<rustc_middle[62e623a39719183a]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[62e623a39719183a]::ty::context::TyCtxt, (), core[1d8afb50f4e7a45]::result::Result<(), rustc_errors[6e875e03adfb8e66]::ErrorGuaranteed>>
  94:     0x7fcdbe25285c - rustc_query_system[ef279d85abd3e98b]::query::plumbing::try_execute_query::<rustc_query_impl[934f89a4ce3db597]::plumbing::QueryCtxt, rustc_query_system[ef279d85abd3e98b]::query::caches::DefaultCache<(), core[1d8afb50f4e7a45]::result::Result<(), rustc_errors[6e875e03adfb8e66]::ErrorGuaranteed>>>
  95:     0x7fcdbe2522cb - rustc_query_system[ef279d85abd3e98b]::query::plumbing::get_query::<rustc_query_impl[934f89a4ce3db597]::queries::analysis, rustc_query_impl[934f89a4ce3db597]::plumbing::QueryCtxt>
  96:     0x7fcdbdcdbe4d - <rustc_interface[d2ac3f3b3bfa4614]::passes::QueryContext>::enter::<rustc_driver[153e38e44d5410f9]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[1d8afb50f4e7a45]::result::Result<(), rustc_errors[6e875e03adfb8e66]::ErrorGuaranteed>>
  97:     0x7fcdbdcd809f - <rustc_interface[d2ac3f3b3bfa4614]::interface::Compiler>::enter::<rustc_driver[153e38e44d5410f9]::run_compiler::{closure#1}::{closure#2}, core[1d8afb50f4e7a45]::result::Result<core[1d8afb50f4e7a45]::option::Option<rustc_interface[d2ac3f3b3bfa4614]::queries::Linker>, rustc_errors[6e875e03adfb8e66]::ErrorGuaranteed>>
  98:     0x7fcdbdcd30b8 - rustc_span[cdf434c68ce75247]::with_source_map::<core[1d8afb50f4e7a45]::result::Result<(), rustc_errors[6e875e03adfb8e66]::ErrorGuaranteed>, rustc_interface[d2ac3f3b3bfa4614]::interface::run_compiler<core[1d8afb50f4e7a45]::result::Result<(), rustc_errors[6e875e03adfb8e66]::ErrorGuaranteed>, rustc_driver[153e38e44d5410f9]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  99:     0x7fcdbdcd2ba5 - <scoped_tls[2ec6d95ce5fd8213]::ScopedKey<rustc_span[cdf434c68ce75247]::SessionGlobals>>::set::<rustc_interface[d2ac3f3b3bfa4614]::interface::run_compiler<core[1d8afb50f4e7a45]::result::Result<(), rustc_errors[6e875e03adfb8e66]::ErrorGuaranteed>, rustc_driver[153e38e44d5410f9]::run_compiler::{closure#1}>::{closure#0}, core[1d8afb50f4e7a45]::result::Result<(), rustc_errors[6e875e03adfb8e66]::ErrorGuaranteed>>
 100:     0x7fcdbdcd2192 - std[2b5db76a972e300f]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d2ac3f3b3bfa4614]::util::run_in_thread_pool_with_globals<rustc_interface[d2ac3f3b3bfa4614]::interface::run_compiler<core[1d8afb50f4e7a45]::result::Result<(), rustc_errors[6e875e03adfb8e66]::ErrorGuaranteed>, rustc_driver[153e38e44d5410f9]::run_compiler::{closure#1}>::{closure#0}, core[1d8afb50f4e7a45]::result::Result<(), rustc_errors[6e875e03adfb8e66]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1d8afb50f4e7a45]::result::Result<(), rustc_errors[6e875e03adfb8e66]::ErrorGuaranteed>>
 101:     0x7fcdbdcd1ea8 - <<std[2b5db76a972e300f]::thread::Builder>::spawn_unchecked_<rustc_interface[d2ac3f3b3bfa4614]::util::run_in_thread_pool_with_globals<rustc_interface[d2ac3f3b3bfa4614]::interface::run_compiler<core[1d8afb50f4e7a45]::result::Result<(), rustc_errors[6e875e03adfb8e66]::ErrorGuaranteed>, rustc_driver[153e38e44d5410f9]::run_compiler::{closure#1}>::{closure#0}, core[1d8afb50f4e7a45]::result::Result<(), rustc_errors[6e875e03adfb8e66]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1d8afb50f4e7a45]::result::Result<(), rustc_errors[6e875e03adfb8e66]::ErrorGuaranteed>>::{closure#1} as core[1d8afb50f4e7a45]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
 102:     0x7fcdbf813c43 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h0c8cfd64ff347c2f
                               at /rustc/70f8737b2f5d3bf7d6b784fad00b663b7ff9feda/library/alloc/src/boxed.rs:2000:9
 103:     0x7fcdbf813c43 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h5243ef33e0bc55ac
                               at /rustc/70f8737b2f5d3bf7d6b784fad00b663b7ff9feda/library/alloc/src/boxed.rs:2000:9
 104:     0x7fcdbf813c43 - std::sys::unix::thread::Thread::new::thread_start::h248bc665ee6b9bda
                               at /rustc/70f8737b2f5d3bf7d6b784fad00b663b7ff9feda/library/std/src/sys/unix/thread.rs:108:17
 105:     0x7fcdbb48cded - start_thread
 106:     0x7fcdbb512370 - __GI___clone3
 107:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (70f8737b2 2022-11-23) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C embed-bitcode=no -C debuginfo=2 -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck] type-checking `acl::<impl at crates/shadowsocks-service/src/acl/mod.rs:327:1: 327:19>::check_target_bypassed`
#1 [thir_body] building THIR for `acl::<impl at crates/shadowsocks-service/src/acl/mod.rs:327:1: 327:19>::check_target_bypassed`
#2 [mir_built] building MIR for `acl::<impl at crates/shadowsocks-service/src/acl/mod.rs:327:1: 327:19>::check_target_bypassed`
#3 [unsafety_check_result] unsafety-checking `acl::<impl at crates/shadowsocks-service/src/acl/mod.rs:327:1: 327:19>::check_target_bypassed`
#4 [mir_const] preparing `acl::<impl at crates/shadowsocks-service/src/acl/mod.rs:327:1: 327:19>::check_target_bypassed` for borrow checking
#5 [mir_promoted] processing MIR for `acl::<impl at crates/shadowsocks-service/src/acl/mod.rs:327:1: 327:19>::check_target_bypassed`
#6 [mir_borrowck] borrow-checking `acl::<impl at crates/shadowsocks-service/src/acl/mod.rs:327:1: 327:19>::check_target_bypassed`
#7 [type_of] computing type of `acl::<impl at crates/shadowsocks-service/src/acl/mod.rs:327:1: 327:19>::check_target_bypassed::{opaque#0}`
#8 [check_mod_item_types] checking item types in module `acl`
#9 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `shadowsocks-service`
