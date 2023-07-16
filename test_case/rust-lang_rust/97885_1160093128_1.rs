
error: internal compiler error: compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:856:33: unexpected path: def=app_definition_validator::AppDefinitionValidator substs=['_#0r] path=Path { span: core10-validation/src/app_definition_validator.rs:206:23: 206:27 (#0), res: SelfTy { trait_: None, alias
_to: Some((DefId(0:161 ~ core10_validation[4b7e]::app_definition_validator::{impl#0}), false)) }, segments: [PathSegment { ident: Self#0, hir_id: Some(HirId { owner: DefId(0:181 ~ core10_validation[4b7e]::app_definition_validator::{impl#0}::find_exe_path_merges), local_id: 6 }), res: Some(SelfTy { trait_: None,
 alias_to: Some((DefId(0:161 ~ core10_validation[4b7e]::app_definition_validator::{impl#0}), false)) }), args: None, infer_args: true }] }

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/5435ed6916a59e8d5acba2149316a841c3905cbd/compiler/rustc_errors/src/lib.rs:1334:9
stack backtrace:
   0:     0x7f8f176d309d - std::backtrace_rs::backtrace::libunwind::trace::hee7943ada6bad29e
                               at /rustc/5435ed6916a59e8d5acba2149316a841c3905cbd/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f8f176d309d - std::backtrace_rs::backtrace::trace_unsynchronized::h7ccd83155a46c065
                               at /rustc/5435ed6916a59e8d5acba2149316a841c3905cbd/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f8f176d309d - std::sys_common::backtrace::_print_fmt::hfe24db8e4118fad6
                               at /rustc/5435ed6916a59e8d5acba2149316a841c3905cbd/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7f8f176d309d - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7d03ef0a4b68cd6d
                               at /rustc/5435ed6916a59e8d5acba2149316a841c3905cbd/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7f8f1772ee2c - core::fmt::write::hb36860eff00e9ada
                               at /rustc/5435ed6916a59e8d5acba2149316a841c3905cbd/library/core/src/fmt/mod.rs:1196:17
   5:     0x7f8f176c4831 - std::io::Write::write_fmt::ha4850a1b39c307b1
                               at /rustc/5435ed6916a59e8d5acba2149316a841c3905cbd/library/std/src/io/mod.rs:1654:15
   6:     0x7f8f176d5d75 - std::sys_common::backtrace::_print::hf3ecf36d224b16f3
                               at /rustc/5435ed6916a59e8d5acba2149316a841c3905cbd/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7f8f176d5d75 - std::sys_common::backtrace::print::ha31789107cdf7920
                               at /rustc/5435ed6916a59e8d5acba2149316a841c3905cbd/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7f8f176d5d75 - std::panicking::default_hook::{{closure}}::h1ca420da35ab94ba
                               at /rustc/5435ed6916a59e8d5acba2149316a841c3905cbd/library/std/src/panicking.rs:295:22
   9:     0x7f8f176d5a96 - std::panicking::default_hook::hf7cfd911ca8444f2
                               at /rustc/5435ed6916a59e8d5acba2149316a841c3905cbd/library/std/src/panicking.rs:314:9
  10:     0x7f8f17eab911 - rustc_driver[9b2cc5dd0ee5154e]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f8f0378a4f3 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h6de3841f3a1106fa
                               at /rustc/5435ed6916a59e8d5acba2149316a841c3905cbd/library/alloc/src/boxed.rs:1965:9
  12:     0x7f8f03779cbd - proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}::h4325c97845b600e4
                               at /rustc/5435ed6916a59e8d5acba2149316a841c3905cbd/library/proc_macro/src/bridge/client.rs:339:21
  13:     0x7f8f176d644a - std::panicking::rust_panic_with_hook::h8b0eedba7af397b5
                               at /rustc/5435ed6916a59e8d5acba2149316a841c3905cbd/library/std/src/panicking.rs:702:17
  14:     0x7f8f18f47191 - std[a0eb98a96cd1d67b]::panicking::begin_panic::<rustc_errors[f2e3be3759aa4428]::ExplicitBug>::{closure#0}
  15:     0x7f8f18f46ba6 - std[a0eb98a96cd1d67b]::sys_common::backtrace::__rust_end_short_backtrace::<std[a0eb98a96cd1d67b]::panicking::begin_panic<rustc_errors[f2e3be3759aa4428]::ExplicitBug>::{closure#0}, !>
  16:     0x7f8f18ec3476 - std[a0eb98a96cd1d67b]::panicking::begin_panic::<rustc_errors[f2e3be3759aa4428]::ExplicitBug>
  17:     0x7f8f18ee9836 - std[a0eb98a96cd1d67b]::panic::panic_any::<rustc_errors[f2e3be3759aa4428]::ExplicitBug>
  18:     0x7f8f18ee92d5 - <rustc_errors[f2e3be3759aa4428]::HandlerInner>::bug::<&alloc[c44c0bf1429d9201]::string::String>
  19:     0x7f8f18ee7630 - <rustc_errors[f2e3be3759aa4428]::Handler>::bug::<&alloc[c44c0bf1429d9201]::string::String>
  20:     0x7f8f18f4412d - rustc_middle[8ca1ffa4f0d07531]::ty::context::tls::with_opt::<rustc_middle[8ca1ffa4f0d07531]::util::bug::opt_span_bug_fmt<rustc_span[e19345dc80a27759]::span_encoding::Span>::{closure#0}, ()>
  21:     0x7f8f18f44226 - rustc_middle[8ca1ffa4f0d07531]::util::bug::opt_span_bug_fmt::<rustc_span[e19345dc80a27759]::span_encoding::Span>
  22:     0x7f8f18f441a3 - rustc_middle[8ca1ffa4f0d07531]::util::bug::bug_fmt
  23:     0x7f8f18e197cb - <rustc_infer[1b92be4cd5a18007]::infer::error_reporting::need_type_info::FindInferSourceVisitor>::path_inferred_subst_iter
  24:     0x7f8f18e19d53 - <rustc_infer[1b92be4cd5a18007]::infer::error_reporting::need_type_info::FindInferSourceVisitor as rustc_hir[3e81ab229655f335]::intravisit::Visitor>::visit_expr
  25:     0x7f8f18e19bdc - <rustc_infer[1b92be4cd5a18007]::infer::error_reporting::need_type_info::FindInferSourceVisitor as rustc_hir[3e81ab229655f335]::intravisit::Visitor>::visit_expr
  26:     0x7f8f18e19bc7 - <rustc_infer[1b92be4cd5a18007]::infer::error_reporting::need_type_info::FindInferSourceVisitor as rustc_hir[3e81ab229655f335]::intravisit::Visitor>::visit_expr
  27:     0x7f8f18e336d1 - rustc_hir[3e81ab229655f335]::intravisit::walk_expr::<rustc_infer[1b92be4cd5a18007]::infer::error_reporting::need_type_info::FindInferSourceVisitor>
  28:     0x7f8f18e19bee - <rustc_infer[1b92be4cd5a18007]::infer::error_reporting::need_type_info::FindInferSourceVisitor as rustc_hir[3e81ab229655f335]::intravisit::Visitor>::visit_expr
  29:     0x7f8f18e302bc - rustc_hir[3e81ab229655f335]::intravisit::walk_local::<rustc_infer[1b92be4cd5a18007]::infer::error_reporting::need_type_info::FindInferSourceVisitor>
  30:     0x7f8f18e19858 - <rustc_infer[1b92be4cd5a18007]::infer::error_reporting::need_type_info::FindInferSourceVisitor as rustc_hir[3e81ab229655f335]::intravisit::Visitor>::visit_local
  31:     0x7f8f18e30107 - rustc_hir[3e81ab229655f335]::intravisit::walk_block::<rustc_infer[1b92be4cd5a18007]::infer::error_reporting::need_type_info::FindInferSourceVisitor>
  32:     0x7f8f18e19bee - <rustc_infer[1b92be4cd5a18007]::infer::error_reporting::need_type_info::FindInferSourceVisitor as rustc_hir[3e81ab229655f335]::intravisit::Visitor>::visit_expr
  33:     0x7f8f18dd8a74 - <rustc_infer[1b92be4cd5a18007]::infer::InferCtxt>::emit_inference_failure_err
  34:     0x7f8f19384ce4 - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::structurally_resolved_type
  35:     0x7f8f193c9edd - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::lookup_method
  36:     0x7f8f193bb59f - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_expr_kind
  37:     0x7f8f193ba230 - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  38:     0x7f8f194fa538 - rustc_typeck[65f6cc0eda670a77]::check::check::check_fn
  39:     0x7f8f193b920d - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_expr_closure
  40:     0x7f8f193bd926 - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_expr_kind
  41:     0x7f8f19385ab3 - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_argument_types
  42:     0x7f8f193bbe44 - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_expr_kind
  43:     0x7f8f193ba230 - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  44:     0x7f8f193bb53c - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_expr_kind
  45:     0x7f8f193ba230 - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  46:     0x7f8f193a7866 - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_stmt
  47:     0x7f8f193a8ea7 - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  48:     0x7f8f193bb4e5 - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_expr_kind
  49:     0x7f8f193ba230 - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  50:     0x7f8f194fa538 - rustc_typeck[65f6cc0eda670a77]::check::check::check_fn
  51:     0x7f8f193b920d - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_expr_closure
  52:     0x7f8f193bd926 - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_expr_kind
  53:     0x7f8f19385ab3 - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_argument_types
  54:     0x7f8f193bbe44 - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_expr_kind
  55:     0x7f8f193ba230 - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  56:     0x7f8f193a7d99 - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_stmt
  57:     0x7f8f193a8ea7 - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  58:     0x7f8f193bb4e5 - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_expr_kind
  59:     0x7f8f193ba230 - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  60:     0x7f8f194fa538 - rustc_typeck[65f6cc0eda670a77]::check::check::check_fn
  61:     0x7f8f193b920d - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_expr_closure
  62:     0x7f8f193bd926 - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_expr_kind
  63:     0x7f8f19385ab3 - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_argument_types
  64:     0x7f8f193bbe44 - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_expr_kind
  65:     0x7f8f193ba230 - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  66:     0x7f8f193bb53c - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_expr_kind
  67:     0x7f8f193ba230 - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  68:     0x7f8f193a7d99 - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_stmt
  69:     0x7f8f193a8edf - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  70:     0x7f8f193bb4e5 - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_expr_kind
  71:     0x7f8f193ba230 - <rustc_typeck[65f6cc0eda670a77]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  72:     0x7f8f194fa538 - rustc_typeck[65f6cc0eda670a77]::check::check::check_fn
  73:     0x7f8f1946ed67 - <rustc_infer[1b92be4cd5a18007]::infer::InferCtxtBuilder>::enter::<&rustc_middle[8ca1ffa4f0d07531]::ty::context::TypeckResults, <rustc_typeck[65f6cc0eda670a77]::check::inherited::InheritedBuilder>::enter<rustc_typeck[65f6cc0eda670a77]::check::typeck_with_fallback<rustc_typeck[65f6cc0ed
a670a77]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[8ca1ffa4f0d07531]::ty::context::TypeckResults>::{closure#0}>
  74:     0x7f8f1940300a - rustc_typeck[65f6cc0eda670a77]::check::typeck
  75:     0x7f8f1a501830 - <rustc_query_system[e792b288446fca87]::dep_graph::graph::DepGraph<rustc_middle[8ca1ffa4f0d07531]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[8ca1ffa4f0d07531]::ty::context::TyCtxt, rustc_span[e19345dc80a27759]::def_id::LocalDefId, &rustc_middle[8ca1ffa4f0d07531]::ty::con
text::TypeckResults>
  76:     0x7f8f19a2b0b0 - rustc_query_system[e792b288446fca87]::query::plumbing::try_execute_query::<rustc_query_impl[f4d741971085f907]::plumbing::QueryCtxt, rustc_query_system[e792b288446fca87]::query::caches::DefaultCache<rustc_span[e19345dc80a27759]::def_id::LocalDefId, &rustc_middle[8ca1ffa4f0d07531]::ty::
context::TypeckResults>>
  77:     0x7f8f1995684e - <rustc_query_impl[f4d741971085f907]::Queries as rustc_middle[8ca1ffa4f0d07531]::ty::query::QueryEngine>::typeck
  78:     0x7f8f194031a2 - rustc_typeck[65f6cc0eda670a77]::check::typeck
  79:     0x7f8f1a501830 - <rustc_query_system[e792b288446fca87]::dep_graph::graph::DepGraph<rustc_middle[8ca1ffa4f0d07531]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[8ca1ffa4f0d07531]::ty::context::TyCtxt, rustc_span[e19345dc80a27759]::def_id::LocalDefId, &rustc_middle[8ca1ffa4f0d07531]::ty::con
text::TypeckResults>
  80:     0x7f8f19a2b0b0 - rustc_query_system[e792b288446fca87]::query::plumbing::try_execute_query::<rustc_query_impl[f4d741971085f907]::plumbing::QueryCtxt, rustc_query_system[e792b288446fca87]::query::caches::DefaultCache<rustc_span[e19345dc80a27759]::def_id::LocalDefId, &rustc_middle[8ca1ffa4f0d07531]::ty::
context::TypeckResults>>
  81:     0x7f8f1995684e - <rustc_query_impl[f4d741971085f907]::Queries as rustc_middle[8ca1ffa4f0d07531]::ty::query::QueryEngine>::typeck
  82:     0x7f8f194c1dc8 - <rustc_middle[8ca1ffa4f0d07531]::hir::map::Map>::par_body_owners::<rustc_typeck[65f6cc0eda670a77]::check::typeck_item_bodies::{closure#0}>
  83:     0x7f8f1a2bf70c - rustc_typeck[65f6cc0eda670a77]::check::typeck_item_bodies
  84:     0x7f8f1a521034 - <rustc_query_system[e792b288446fca87]::dep_graph::graph::DepGraph<rustc_middle[8ca1ffa4f0d07531]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[8ca1ffa4f0d07531]::ty::context::TyCtxt, (), ()>
  85:     0x7f8f1a5cc0e5 - rustc_query_system[e792b288446fca87]::query::plumbing::try_execute_query::<rustc_query_impl[f4d741971085f907]::plumbing::QueryCtxt, rustc_query_system[e792b288446fca87]::query::caches::DefaultCache<(), ()>>
  86:     0x7f8f1a5f6371 - rustc_query_system[e792b288446fca87]::query::plumbing::get_query::<rustc_query_impl[f4d741971085f907]::queries::typeck_item_bodies, rustc_query_impl[f4d741971085f907]::plumbing::QueryCtxt>
  87:     0x7f8f1a2f60b3 - <rustc_session[e99e5395caf9b62]::session::Session>::time::<(), rustc_typeck[65f6cc0eda670a77]::check_crate::{closure#7}>
  88:     0x7f8f1a2e2cdb - rustc_typeck[65f6cc0eda670a77]::check_crate
  89:     0x7f8f1a09f587 - rustc_interface[2808bdc579fb1dcb]::passes::analysis
  90:     0x7f8f1a51c315 - <rustc_query_system[e792b288446fca87]::dep_graph::graph::DepGraph<rustc_middle[8ca1ffa4f0d07531]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[8ca1ffa4f0d07531]::ty::context::TyCtxt, (), core[9a5575405027e362]::result::Result<(), rustc_errors[f2e3be3759aa4428]::ErrorGuaran
teed>>
  91:     0x7f8f1a5c154d - rustc_query_system[e792b288446fca87]::query::plumbing::try_execute_query::<rustc_query_impl[f4d741971085f907]::plumbing::QueryCtxt, rustc_query_system[e792b288446fca87]::query::caches::DefaultCache<(), core[9a5575405027e362]::result::Result<(), rustc_errors[f2e3be3759aa4428]::ErrorGua
ranteed>>>
  92:     0x7f8f1a60892e - rustc_query_system[e792b288446fca87]::query::plumbing::get_query::<rustc_query_impl[f4d741971085f907]::queries::analysis, rustc_query_impl[f4d741971085f907]::plumbing::QueryCtxt>
  93:     0x7f8f1a05ca47 - <rustc_interface[2808bdc579fb1dcb]::passes::QueryContext>::enter::<rustc_driver[9b2cc5dd0ee5154e]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[9a5575405027e362]::result::Result<(), rustc_errors[f2e3be3759aa4428]::ErrorGuaranteed>>
  94:     0x7f8f1a046bbf - <rustc_interface[2808bdc579fb1dcb]::interface::Compiler>::enter::<rustc_driver[9b2cc5dd0ee5154e]::run_compiler::{closure#1}::{closure#2}, core[9a5575405027e362]::result::Result<core[9a5575405027e362]::option::Option<rustc_interface[2808bdc579fb1dcb]::queries::Linker>, rustc_errors[f2e
3be3759aa4428]::ErrorGuaranteed>>
  95:     0x7f8f1a0701cf - rustc_span[e19345dc80a27759]::with_source_map::<core[9a5575405027e362]::result::Result<(), rustc_errors[f2e3be3759aa4428]::ErrorGuaranteed>, rustc_interface[2808bdc579fb1dcb]::interface::create_compiler_and_run<core[9a5575405027e362]::result::Result<(), rustc_errors[f2e3be3759aa4428]:
:ErrorGuaranteed>, rustc_driver[9b2cc5dd0ee5154e]::run_compiler::{closure#1}>::{closure#1}>
  96:     0x7f8f1a047a52 - <scoped_tls[846b7f23ce5bfed4]::ScopedKey<rustc_span[e19345dc80a27759]::SessionGlobals>>::set::<rustc_interface[2808bdc579fb1dcb]::interface::run_compiler<core[9a5575405027e362]::result::Result<(), rustc_errors[f2e3be3759aa4428]::ErrorGuaranteed>, rustc_driver[9b2cc5dd0ee5154e]::run_co
mpiler::{closure#1}>::{closure#0}, core[9a5575405027e362]::result::Result<(), rustc_errors[f2e3be3759aa4428]::ErrorGuaranteed>>
  97:     0x7f8f1a05d0cf - std[a0eb98a96cd1d67b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2808bdc579fb1dcb]::util::run_in_thread_pool_with_globals<rustc_interface[2808bdc579fb1dcb]::interface::run_compiler<core[9a5575405027e362]::result::Result<(), rustc_errors[f2e3be3759aa4428]::E
rrorGuaranteed>, rustc_driver[9b2cc5dd0ee5154e]::run_compiler::{closure#1}>::{closure#0}, core[9a5575405027e362]::result::Result<(), rustc_errors[f2e3be3759aa4428]::ErrorGuaranteed>>::{closure#0}, core[9a5575405027e362]::result::Result<(), rustc_errors[f2e3be3759aa4428]::ErrorGuaranteed>>
  98:     0x7f8f1a05d239 - <<std[a0eb98a96cd1d67b]::thread::Builder>::spawn_unchecked_<rustc_interface[2808bdc579fb1dcb]::util::run_in_thread_pool_with_globals<rustc_interface[2808bdc579fb1dcb]::interface::run_compiler<core[9a5575405027e362]::result::Result<(), rustc_errors[f2e3be3759aa4428]::ErrorGuaranteed>, 
rustc_driver[9b2cc5dd0ee5154e]::run_compiler::{closure#1}>::{closure#0}, core[9a5575405027e362]::result::Result<(), rustc_errors[f2e3be3759aa4428]::ErrorGuaranteed>>::{closure#0}, core[9a5575405027e362]::result::Result<(), rustc_errors[f2e3be3759aa4428]::ErrorGuaranteed>>::{closure#1} as core[9a5575405027e362]:
:ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  99:     0x7f8f176e0373 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hed7040088a8f20af
                               at /rustc/5435ed6916a59e8d5acba2149316a841c3905cbd/library/alloc/src/boxed.rs:1951:9
 100:     0x7f8f176e0373 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h74b0534bb454589b
                               at /rustc/5435ed6916a59e8d5acba2149316a841c3905cbd/library/alloc/src/boxed.rs:1951:9
 101:     0x7f8f176e0373 - std::sys::unix::thread::Thread::new::thread_start::hf83752a2adec8b03
                               at /rustc/5435ed6916a59e8d5acba2149316a841c3905cbd/library/std/src/sys/unix/thread.rs:108:17
 102:     0x7f8f175fe609 - start_thread
                               at /build/glibc-sMfBJT/glibc-2.31/nptl/pthread_create.c:477:8
 103:     0x7f8f17521163 - clone
 104:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (5435ed691 2022-06-07) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type cdylib --crate-type rlib -C embed-bitcode=no -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck] type-checking `app_definition_validator::<impl at core10-validation/src/app_definition_validator.rs:54:1: 272:2>::find_exe_path_merges`
#1 [typeck] type-checking `app_definition_validator::<impl at core10-validation/src/app_definition_validator.rs:54:1: 272:2>::find_exe_path_merges::{closure#0}`
#2 [typeck_item_bodies] type-checking all item bodies
#3 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `core10-validation`
