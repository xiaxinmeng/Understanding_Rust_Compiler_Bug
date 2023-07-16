rust
error: internal compiler error: compiler/rustc_trait_selection/src/traits/specialize/mod.rs:165:17: failed to fully normalize <T as Trait<<T as std::iter::Iterator>::Item>>: [FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<T as std::iter::Iterator>), []), depth=0),Unimplemented)]

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1007:9
stack backtrace:
   0: std::panicking::begin_panic
   1: std::panic::panic_any
   2: rustc_errors::HandlerInner::bug
   3: rustc_errors::Handler::bug
   4: rustc_middle::ty::context::tls::with_opt
   5: rustc_middle::util::bug::opt_span_bug_fmt
   6: rustc_middle::util::bug::bug_fmt
   7: rustc_infer::infer::InferCtxtBuilder::enter
   8: rustc_trait_selection::traits::specialize::specializes
   9: rustc_query_system::query::plumbing::get_query_impl
  10: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::specializes
  11: rustc_infer::infer::InferCtxtBuilder::enter
  12: <rustc_middle::traits::specialization_graph::Children as rustc_trait_selection::traits::specialize::specialization_graph::ChildrenExt>::insert
  13: <rustc_middle::traits::specialization_graph::Graph as rustc_trait_selection::traits::specialize::specialization_graph::GraphExt>::insert
  14: rustc_trait_selection::traits::specialize::specialization_graph_provider
  15: rustc_query_system::query::plumbing::get_query_impl
  16: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::specialization_graph_of
  17: rustc_typeck::coherence::coherent_trait
  18: rustc_query_system::query::plumbing::get_query_impl
  19: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::coherent_trait
  20: rustc_typeck::coherence::check_coherence
  21: rustc_session::session::Session::track_errors
  22: rustc_typeck::check_crate
  23: rustc_interface::passes::analysis
  24: rustc_query_system::query::plumbing::get_query_impl
  25: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
  26: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  27: rustc_span::with_source_map
  28: rustc_interface::interface::create_compiler_and_run
  29: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (406d4a9cc 2021-06-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [specializes] computing whether impls specialize one another
#1 [specialization_graph_of] building specialization graph of trait `Trait`
#2 [coherent_trait] coherence checking all impls of trait `Trait`
#3 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error; 1 warning emitted

error: could not compile `playground`
