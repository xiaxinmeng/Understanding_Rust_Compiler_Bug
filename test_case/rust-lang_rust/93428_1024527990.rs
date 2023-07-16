
[slamb@slamb-workstation ~/git/retina]$ RUST_BACKTRACE=1 cargo +nightly doc
 Documenting retina v0.3.7 (/home/slamb/git/retina)
thread 'rustc' panicked at 'no entry found for key', src/librustdoc/passes/collect_intra_doc_links.rs:929:16
stack backtrace:
   0: rust_begin_unwind
             at /rustc/6abb6385b2cb7249f67b9b3ce7522527767dd907/library/std/src/panicking.rs:577:5
   1: core::panicking::panic_fmt
             at /rustc/6abb6385b2cb7249f67b9b3ce7522527767dd907/library/core/src/panicking.rs:135:14
   2: core::panicking::panic_display
             at /rustc/6abb6385b2cb7249f67b9b3ce7522527767dd907/library/core/src/panicking.rs:65:5
   3: core::panicking::panic_str
             at /rustc/6abb6385b2cb7249f67b9b3ce7522527767dd907/library/core/src/panicking.rs:56:5
   4: core::option::expect_failed
             at /rustc/6abb6385b2cb7249f67b9b3ce7522527767dd907/library/core/src/option.rs:1840:5
   5: rustdoc::passes::collect_intra_doc_links::resolve_associated_trait_item
   6: <rustdoc::passes::collect_intra_doc_links::LinkCollector>::resolve_associated_item
   7: <rustdoc::passes::collect_intra_doc_links::LinkCollector>::resolve
   8: <rustdoc::passes::collect_intra_doc_links::LinkCollector>::resolve_link
   9: <rustdoc::passes::collect_intra_doc_links::LinkCollector as rustdoc::visit::DocVisitor>::visit_item
  10: <rustdoc::passes::collect_intra_doc_links::LinkCollector as rustdoc::visit::DocVisitor>::visit_inner_recur
  11: <rustdoc::passes::collect_intra_doc_links::LinkCollector as rustdoc::visit::DocVisitor>::visit_item
  12: <rustdoc::passes::collect_intra_doc_links::LinkCollector as rustdoc::visit::DocVisitor>::visit_inner_recur
  13: <rustdoc::passes::collect_intra_doc_links::LinkCollector as rustdoc::visit::DocVisitor>::visit_item
  14: rustdoc::passes::collect_intra_doc_links::collect_intra_doc_links
  15: <rustc_session::session::Session>::time::<rustdoc::clean::types::Crate, rustdoc::core::run_global_ctxt::{closure#8}>
  16: rustdoc::core::run_global_ctxt
  17: <rustc_session::session::Session>::time::<(rustdoc::clean::types::Crate, rustdoc::config::RenderOptions, rustdoc::formats::cache::Cache), rustdoc::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#0}>
  18: <rustc_interface::interface::Compiler>::enter::<rustdoc::main_options::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
  19: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustdoc::main_options::{closure#0}>::{closure#1}>
  20: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorReported>, rustdoc::main_options::{closure#0}>
  21: rustdoc::main_options
  22: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustdoc::main_args::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
