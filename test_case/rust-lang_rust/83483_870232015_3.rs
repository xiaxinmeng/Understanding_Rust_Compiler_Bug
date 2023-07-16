
error: internal compiler error: compiler/rustc_typeck/src/collect/type_of.rs:407:21: compute_type_of_item: unexpected item type: Trait(No, Normal, Generics { params: [], where_clause: WhereClause { predicates: [], span: mutant.rs:3:21: 3:21 (#0) }, span: mutant.rs:3:21: 3:21 (#0) }, [], [])
 --> mutant.rs:3:1
  |
3 | trait PrinterSupport {}
  | ^^^^^^^^^^^^^^^^^^^^^^^

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/a435b49e86d16e98dcc6595dd471f95e823f41aa/compiler/rustc_errors/src/lib.rs:953:9
stack backtrace:
   0: std::panicking::begin_panic
   1: std::panic::panic_any
   2: rustc_errors::HandlerInner::span_bug
   3: rustc_errors::Handler::span_bug
   4: rustc_middle::ty::context::tls::with_opt
   5: rustc_middle::util::bug::opt_span_bug_fmt
   6: rustc_middle::util::bug::span_bug_fmt
   7: rustc_typeck::collect::type_of::type_of
   8: rustc_query_system::query::plumbing::get_query_impl
   9: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::type_of
  10: rustc_symbol_mangling::legacy::mangle
  11: rustc_symbol_mangling::symbol_name_provider
  12: rustc_query_system::query::plumbing::get_query_impl
  13: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::symbol_name
  14: rustc_symbol_mangling::test::SymbolNamesTest::process_attrs
  15: rustc_hir::hir::Crate::visit_all_item_likes
  16: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  17: rustc_symbol_mangling::test::report_symbol_names
  18: rustc_interface::passes::QueryContext::enter
  19: rustc_interface::queries::Queries::ongoing_codegen
  20: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  21: rustc_span::with_source_map
  22: rustc_interface::interface::create_compiler_and_run
  23: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (a435b49e8 2021-06-28) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type staticlib

query stack during panic:
#0 [type_of] computing type of `PrinterSupport`
#1 [symbol_name] computing the symbol for `PrinterSupport`
end of query stack
error: aborting due to previous error
