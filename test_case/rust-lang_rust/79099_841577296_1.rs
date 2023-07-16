
   Compiling playground v0.0.1 (/playground)
warning: the feature `impl_trait_in_bindings` is incomplete and may not be safe to use and/or cause compiler crashes
 --> src/main.rs:1:12
  |
1 | #![feature(impl_trait_in_bindings)]
  |            ^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(incomplete_features)]` on by default
  = note: see issue #63065 <https://github.com/rust-lang/rust/issues/63065> for more information

error[E0277]: expected a `Fn<()>` closure, found `impl Future`
 --> src/main.rs:2:10
  |
2 | const _: impl Fn() = async { 0 };
  |          ^^^^^^^^^ expected an `Fn<()>` closure, found `impl Future`
  |
  = help: the trait `Fn<()>` is not implemented for `impl Future`
  = note: wrap the `impl Future` in a closure with no arguments: `|| { /* code */ }`

error: internal compiler error: compiler/rustc_mir/src/borrow_check/universal_regions.rs:533:26: expected defining type for `DefId(0:5 ~ playground[b426]::_::{closure#0})`: `[type error]`
 --> src/main.rs:2:28
  |
2 | const _: impl Fn() = async { 0 };
  |                            ^^^^^

thread 'rustc' panicked at 'Box<Any>', /rustc/6d395a1c2946c79966490f5b1e6e619d3a713e6b/library/std/src/panic.rs:59:5
stack backtrace:
   0: std::panicking::begin_panic
   1: std::panic::panic_any
   2: rustc_errors::HandlerInner::span_bug
   3: rustc_errors::Handler::span_bug
   4: rustc_middle::ty::context::tls::with_opt
   5: rustc_middle::util::bug::opt_span_bug_fmt
   6: rustc_middle::util::bug::span_bug_fmt
   7: rustc_mir::borrow_check::universal_regions::UniversalRegions::new
   8: rustc_mir::borrow_check::nll::replace_regions_in_mir
   9: rustc_mir::borrow_check::do_mir_borrowck
  10: rustc_infer::infer::InferCtxtBuilder::enter
  11: rustc_mir::borrow_check::mir_borrowck
  12: core::ops::function::FnOnce::call_once
  13: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  14: rustc_data_structures::stack::ensure_sufficient_stack
  15: rustc_query_system::query::plumbing::force_query_with_job
  16: rustc_query_system::query::plumbing::get_query_impl
  17: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_borrowck
  18: rustc_typeck::collect::type_of::find_opaque_ty_constraints::ConstraintLocator::check
  19: <rustc_typeck::collect::type_of::find_opaque_ty_constraints::ConstraintLocator as rustc_hir::intravisit::Visitor>::visit_expr
  20: rustc_hir::intravisit::walk_expr
  21: rustc_hir::intravisit::Visitor::visit_nested_body
  22: rustc_typeck::collect::type_of::type_of
  23: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  24: rustc_data_structures::stack::ensure_sufficient_stack
  25: rustc_query_system::query::plumbing::force_query_with_job
  26: rustc_query_system::query::plumbing::get_query_impl
  27: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::type_of
  28: rustc_typeck::check::check::check_item_type
  29: rustc_middle::hir::map::Map::visit_item_likes_in_module
  30: rustc_typeck::check::check::check_mod_item_types
  31: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  32: rustc_query_system::query::plumbing::force_query_with_job
  33: rustc_query_system::query::plumbing::get_query_impl
  34: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::check_mod_item_types
  35: rustc_session::utils::<impl rustc_session::session::Session>::time
  36: rustc_typeck::check_crate
  37: rustc_interface::passes::analysis
  38: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  39: rustc_data_structures::stack::ensure_sufficient_stack
  40: rustc_query_system::query::plumbing::force_query_with_job
  41: rustc_query_system::query::plumbing::get_query_impl
  42: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
  43: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  44: rustc_span::with_source_map
  45: rustc_interface::interface::create_compiler_and_run
  46: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (6d395a1c2 2021-05-13) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [mir_borrowck] borrow-checking `_::{closure#0}`
#1 [type_of] computing type of `_::{opaque#0}`
#2 [check_mod_item_types] checking item types in top-level module
#3 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to 2 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0277`.
error: could not compile `playground`

To learn more, run the command again with --verbose.
