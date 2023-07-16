
error[E0773]: attempted to define built-in macro more than once
    --> /home/imperio/rust/rust/library/core/src/macros/mod.rs:1111:5
     |
1111 | /     macro_rules! cfg {
1112 | |         ($($cfg:tt)*) => {
1113 | |             /* compiler built-in */
1114 | |         };
1115 | |     }
     | |_____^
     |
note: previously defined here
    --> /home/imperio/rust/rust/library/core/src/macros/mod.rs:1111:5
     |
1111 | /     macro_rules! cfg {
1112 | |         ($($cfg:tt)*) => {
1113 | |             /* compiler built-in */
1114 | |         };
1115 | |     }
     | |_____^

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1028:27
stack backtrace:
   0: std::panicking::begin_panic
   1: rustc_errors::HandlerInner::emit_diagnostic
   2: rustc_errors::diagnostic_builder::DiagnosticBuilder::emit
   3: rustc_resolve::macros::<impl rustc_resolve::Resolver>::compile_macro
   4: rustc_resolve::build_reduced_graph::<impl rustc_resolve::Resolver>::get_macro_by_def_id
   5: rustc_resolve::macros::<impl rustc_resolve::Resolver>::check_reserved_macro_name
   6: rustc_resolve::imports::<impl rustc_resolve::Resolver>::try_define
   7: rustc_resolve::build_reduced_graph::<impl rustc_resolve::Resolver>::define
   8: rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor::build_reduced_graph_for_external_crate_res
   9: rustc_resolve::Resolver::resolutions
  10: rustc_resolve::Resolver::resolution
  11: rustc_resolve::imports::<impl rustc_resolve::Resolver>::resolve_ident_in_module_unadjusted_ext
  12: rustc_resolve::Resolver::resolve_path_with_ribs::{{closure}}
  13: rustc_resolve::Resolver::resolve_path_with_ribs
  14: rustc_resolve::Resolver::resolve_str_path_error
  15: rustc_interface::passes::BoxedResolver::access::{{closure}}
  16: rustc_interface::passes::configure_and_expand::{{closure}}
  17: rustc_interface::passes::BoxedResolver::access
  18: rustdoc::passes::collect_intra_doc_links::LinkCollector::resolve_link
  19: <rustdoc::passes::collect_intra_doc_links::LinkCollector as rustdoc::fold::DocFolder>::fold_item
  20: alloc::vec::source_iter_marker::<impl alloc::vec::spec_from_iter::SpecFromIter<T,I> for alloc::vec::Vec<T>>::from_iter
  21: rustdoc::fold::DocFolder::fold_inner_recur
  22: rustdoc::fold::DocFolder::fold_item_recur
  23: <rustdoc::passes::collect_intra_doc_links::LinkCollector as rustdoc::fold::DocFolder>::fold_item
  24: rustdoc::passes::collect_intra_doc_links::collect_intra_doc_links
  25: rustdoc::core::run_global_ctxt
  26: rustc_session::utils::<impl rustc_session::session::Session>::time
  27: rustc_interface::passes::QueryContext::enter
  28: rustc_interface::interface::create_compiler_and_run
  29: rustdoc::main_options
  30: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic
