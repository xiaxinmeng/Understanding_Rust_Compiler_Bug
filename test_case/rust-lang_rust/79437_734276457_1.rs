
error[E0072]: recursive type `Take` has infinite size
 --> /home/kuba/tmp/repro.rs:1:1
  |
1 | struct Take(Self);
  | ^^^^^^^^^^^^----^^
  | |           |
  | |           recursive without indirection
  | recursive type has infinite size
  |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `Take` representable
  |
1 | struct Take(Box<Self>);
  |             ^^^^    ^

thread 'rustc' panicked at 'Called struct_tail_with_normalize on recursive type: Take', compiler/rustc_middle/src/ty/util.rs:260:17
stack backtrace:
   0: rust_begin_unwind
   1: std::panicking::begin_panic_fmt
   2: rustc_middle::ty::util::<impl rustc_middle::ty::context::TyCtxt>::struct_tail_with_normalize
   3: rustc_typeck::check::expectation::Expectation::rvalue_hint
   4: rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_argument_types
   5: rustc_typeck::check::callee::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::confirm_builtin_call
   6: rustc_typeck::check::callee::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_call
   7: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
   8: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
   9: rustc_typeck::check::fn_ctxt::_impl::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::with_breakable_ctxt
  10: rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_block_with_expected
  11: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
  12: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  13: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_return_expr
  14: rustc_typeck::check::check::check_fn
  15: rustc_infer::infer::InferCtxtBuilder::enter
  16: rustc_typeck::check::inherited::InheritedBuilder::enter
  17: rustc_typeck::check::typeck_with_fallback
  18: rustc_typeck::check::typeck
  19: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  20: rustc_data_structures::stack::ensure_sufficient_stack
  21: rustc_query_system::query::plumbing::get_query_impl
  22: rustc_middle::ty::context::TyCtxt::typeck_opt_const_arg
  23: rustc_mir_build::thir::cx::Cx::new
  24: rustc_infer::infer::InferCtxtBuilder::enter
  25: rustc_mir_build::build::mir_built
  26: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_built>::compute
  27: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  28: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  29: rustc_data_structures::stack::ensure_sufficient_stack
  30: rustc_query_system::query::plumbing::get_query_impl
  31: rustc_mir::transform::check_unsafety::unsafety_check_result
  32: core::ops::function::FnOnce::call_once
  33: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::unsafety_check_result>::compute
  34: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  35: rustc_data_structures::stack::ensure_sufficient_stack
  36: rustc_query_system::query::plumbing::get_query_impl
  37: rustc_query_system::query::plumbing::ensure_query_impl
  38: rustc_mir::transform::mir_const
  39: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_const>::compute
  40: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  41: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  42: rustc_data_structures::stack::ensure_sufficient_stack
  43: rustc_query_system::query::plumbing::get_query_impl
  44: rustc_mir::transform::mir_promoted
  45: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_promoted>::compute
  46: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  47: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  48: rustc_data_structures::stack::ensure_sufficient_stack
  49: rustc_query_system::query::plumbing::get_query_impl
  50: rustc_mir::borrow_check::mir_borrowck
  51: core::ops::function::FnOnce::call_once
  52: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_borrowck>::compute
  53: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  54: rustc_data_structures::stack::ensure_sufficient_stack
  55: rustc_query_system::query::plumbing::get_query_impl
  56: rustc_typeck::collect::type_of::type_of
  57: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::type_of>::compute
  58: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  59: rustc_data_structures::stack::ensure_sufficient_stack
  60: rustc_query_system::query::plumbing::get_query_impl
  61: rustc_typeck::check::check::check_item_type
  62: rustc_middle::hir::map::Map::visit_item_likes_in_module
  63: rustc_typeck::check::check::check_mod_item_types
  64: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::check_mod_item_types>::compute
  65: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  66: rustc_data_structures::stack::ensure_sufficient_stack
  67: rustc_query_system::query::plumbing::get_query_impl
  68: rustc_query_system::query::plumbing::ensure_query_impl
  69: rustc_session::utils::<impl rustc_session::session::Session>::time
  70: rustc_typeck::check_crate
  71: rustc_interface::passes::analysis
  72: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
  73: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  74: rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task
  75: rustc_data_structures::stack::ensure_sufficient_stack
  76: rustc_query_system::query::plumbing::get_query_impl
  77: rustc_interface::passes::QueryContext::enter
  78: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  79: rustc_span::with_source_map
  80: rustc_interface::interface::create_compiler_and_run
  81: scoped_tls::ScopedKey<T>::set
  82: rustc_span::with_session_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.50.0-dev running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [typeck] type-checking `foo`
#1 [mir_built] building MIR for `foo`
#2 [unsafety_check_result] unsafety-checking `foo`
#3 [mir_const] processing MIR for `foo`
#4 [mir_promoted] processing `foo`
#5 [mir_borrowck] borrow-checking `foo`
#6 [type_of] computing type of `foo::{opaque#0}`
#7 [check_mod_item_types] checking item types in top-level module
#8 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error

For more information about this error, try `rustc --explain E0072`.
