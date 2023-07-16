
error: internal compiler error: compiler/rustc_borrowck/src/universal_regions.rs:782:36: cannot convert `RePlaceholder(Placeholder { universe: U2, name: BrAnon(1) })` to a region vid

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1146:9
stack backtrace:
   0: std::panicking::begin_panic
   1: std::panic::panic_any
   2: rustc_errors::HandlerInner::bug
   3: rustc_errors::Handler::bug
   4: rustc_middle::ty::context::tls::with_opt
   5: rustc_middle::util::bug::opt_span_bug_fmt
   6: rustc_middle::util::bug::bug_fmt
   7: rustc_borrowck::type_check::free_region_relations::UniversalRegionRelationsBuilder::add_implied_bounds
   8: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once
   9: <core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next
  10: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
  11: rustc_borrowck::type_check::free_region_relations::create
  12: rustc_borrowck::type_check::type_check
  13: rustc_borrowck::nll::compute_regions
  14: rustc_borrowck::do_mir_borrowck
  15: rustc_infer::infer::InferCtxtBuilder::enter
  16: rustc_borrowck::mir_borrowck
  17: core::ops::function::FnOnce::call_once
  18: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  19: rustc_data_structures::stack::ensure_sufficient_stack
  20: rustc_query_system::query::plumbing::try_execute_query
  21: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_borrowck
  22: rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::par_body_owners
  23: rustc_interface::passes::analysis
  24: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  25: rustc_data_structures::stack::ensure_sufficient_stack
  26: rustc_query_system::query::plumbing::try_execute_query
  27: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
  28: rustc_interface::passes::QueryContext::enter
  29: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  30: rustc_span::with_source_map
  31: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (308dffd25 2021-09-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -C opt-level=3 -C embed-bitcode=no -C debuginfo=2 -C debug-assertions=on -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [mir_borrowck] borrow-checking `main::{closure#2}`
#1 [analysis] running analysis passes on this crate
end of query stack
