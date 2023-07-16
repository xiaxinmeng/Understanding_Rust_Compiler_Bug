
error: internal compiler error: compiler/rustc_metadata/src/rmeta/decoder.rs:900:18: get_adt_def called on a non-ADT DefId(250:1108 ~ zebra_consensus[2792]::transaction::Response)

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1146:9
stack backtrace:
   0: std::panicking::begin_panic
   1: std::panic::panic_any
   2: rustc_errors::HandlerInner::bug
   3: rustc_errors::Handler::bug
   4: rustc_middle::ty::context::tls::with_opt
   5: rustc_middle::util::bug::opt_span_bug_fmt
   6: rustc_middle::util::bug::bug_fmt
   7: rustc_metadata::rmeta::decoder::<impl rustc_metadata::creader::CrateMetadataRef>::get_adt_def
   8: rustc_metadata::rmeta::decoder::cstore_impl::provide_extern::adt_def
   9: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  10: rustc_data_structures::stack::ensure_sufficient_stack
  11: rustc_query_system::query::plumbing::try_execute_query
  12: rustc_query_system::query::plumbing::force_query_impl
  13: rustc_query_system::query::plumbing::force_query
  14: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
  15: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
  16: rustc_query_system::query::plumbing::ensure_must_run
  17: rustc_query_system::query::plumbing::get_query
  18: rustc_session::session::Session::track_errors
  19: rustc_typeck::check_crate
  20: rustc_interface::passes::analysis
  21: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  22: rustc_data_structures::stack::ensure_sufficient_stack
  23: rustc_query_system::query::plumbing::try_execute_query
  24: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
  25: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  26: rustc_span::with_source_map
  27: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (d7c97a02d 2021-10-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z share-generics=y -C panic=abort -C embed-bitcode=no -C debuginfo=2 -C linker=clang -C incremental -C link-arg=-fuse-ld=lld --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [adt_def] computing ADT definition for `zebra_consensus::transaction::Response`
#1 [analysis] running analysis passes on this crate
end of query stack
