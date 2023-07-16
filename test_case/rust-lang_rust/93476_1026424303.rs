console
$ RUST_BACKTRACE=1 bundle exec rake doc
rustup run --install nightly cargo doc --workspace
 Documenting artichoke-backend v0.6.0 (/Users/lopopolo/dev/artichoke/artichoke/artichoke-backend)
    Checking artichoke v0.1.0-pre.0 (/Users/lopopolo/dev/artichoke/artichoke)
thread 'rustc' panicked at 'no entry found for key', src/librustdoc/passes/collect_intra_doc_links.rs:929:16
stack backtrace:
   0: _rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::panic_display
   3: core::panicking::panic_str
   4: core::option::expect_failed
   5: rustdoc::passes::collect_intra_doc_links::resolve_associated_trait_item
   6: <rustdoc::passes::collect_intra_doc_links::LinkCollector>::resolve_associated_item
   7: <rustdoc::passes::collect_intra_doc_links::LinkCollector>::resolve
   8: <rustdoc::passes::collect_intra_doc_links::LinkCollector>::resolve_link
   9: <rustdoc::passes::collect_intra_doc_links::LinkCollector as rustdoc::visit::DocVisitor>::visit_item
  10: <rustdoc::passes::collect_intra_doc_links::LinkCollector as rustdoc::visit::DocVisitor>::visit_inner_recur
  11: <rustdoc::passes::collect_intra_doc_links::LinkCollector as rustdoc::visit::DocVisitor>::visit_item
  12: <rustdoc::passes::collect_intra_doc_links::LinkCollector as rustdoc::visit::DocVisitor>::visit_inner_recur
  13: <rustdoc::passes::collect_intra_doc_links::LinkCollector as rustdoc::visit::DocVisitor>::visit_item
  14: <rustdoc::passes::collect_intra_doc_links::LinkCollector as rustdoc::visit::DocVisitor>::visit_inner_recur
  15: <rustdoc::passes::collect_intra_doc_links::LinkCollector as rustdoc::visit::DocVisitor>::visit_item
  16: <rustdoc::passes::collect_intra_doc_links::LinkCollector as rustdoc::visit::DocVisitor>::visit_inner_recur
  17: <rustdoc::passes::collect_intra_doc_links::LinkCollector as rustdoc::visit::DocVisitor>::visit_item
  18: <rustdoc::passes::collect_intra_doc_links::LinkCollector as rustdoc::visit::DocVisitor>::visit_inner_recur
  19: <rustdoc::passes::collect_intra_doc_links::LinkCollector as rustdoc::visit::DocVisitor>::visit_item
  20: rustdoc::passes::collect_intra_doc_links::collect_intra_doc_links
  21: <rustc_session::session::Session>::time::<rustdoc::clean::types::Crate, rustdoc::core::run_global_ctxt::{closure#8}>
  22: rustdoc::core::run_global_ctxt
  23: <rustc_session::session::Session>::time::<(rustdoc::clean::types::Crate, rustdoc::config::RenderOptions, rustdoc::formats::cache::Cache), rustdoc::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#0}>
  24: <rustc_interface::interface::Compiler>::enter::<rustdoc::main_options::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
  25: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustdoc::main_options::{closure#0}>::{closure#1}>
  26: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorReported>, rustdoc::main_options::{closure#0}>
  27: rustdoc::main_options
  28: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustdoc::main_args::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

error: Unrecognized option: 'crate-version'

error: could not document `artichoke-backend`
