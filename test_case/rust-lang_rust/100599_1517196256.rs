
error: internal compiler error: compiler/rustc_metadata/src/creader.rs:352:17: Previously returned E0523 here. See https://github.com/rust-lang/rust/pull/100599 for additional discussion.root.name() = core.

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/compiler/rustc_errors/src/lib.rs:1644:9
stack backtrace:
   0: std::panicking::begin_panic::<rustc_errors::ExplicitBug>
   1: std::panic::panic_any::<rustc_errors::ExplicitBug>
   2: <rustc_errors::HandlerInner>::bug::<&alloc::string::String>
   3: <rustc_errors::Handler>::bug::<&alloc::string::String>
   4: rustc_middle::util::bug::opt_span_bug_fmt::<rustc_span::span_encoding::Span>::{closure#0}
   5: rustc_middle::ty::context::tls::with_opt::<rustc_middle::util::bug::opt_span_bug_fmt<rustc_span::span_encoding::Span>::{closure#0}, !>::{closure#0}
   6: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_opt<rustc_middle::util::bug::opt_span_bug_fmt<rustc_span::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
   7: rustc_middle::util::bug::opt_span_bug_fmt::<rustc_span::span_encoding::Span>
   8: rustc_middle::util::bug::bug_fmt
   9: <rustc_metadata::creader::CrateLoader>::maybe_resolve_crate
  10: <rustc_metadata::creader::CrateLoader>::maybe_resolve_crate
  11: <rustc_metadata::creader::CrateLoader>::resolve_crate
  12: <rustc_metadata::creader::CrateLoader>::process_extern_crate
  13: <rustc_resolve::Resolver>::crate_loader::<core::option::Option<rustc_span::def_id::CrateNum>, <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor>::build_reduced_graph_for_extern_crate::{closure#0}>
  14: <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast::visit::Visitor>::visit_item
  15: <rustc_resolve::Resolver as rustc_expand::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  16: <rustc_expand::expand::MacroExpander>::fully_expand_fragment
  17: <rustc_expand::expand::MacroExpander>::expand_crate
  18: <rustc_session::session::Session>::time::<rustc_ast::ast::Crate, rustc_interface::passes::configure_and_expand::{closure#1}>
  19: rustc_interface::passes::resolver_for_lowering
  20: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::resolver_for_lowering, rustc_query_impl::plumbing::QueryCtxt>
  21: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::resolver_for_lowering
  22: <rustc_middle::ty::context::GlobalCtxt>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#2}, &rustc_data_structures::steal::Steal<(rustc_middle::ty::ResolverAstLowering, alloc::rc::Rc<rustc_ast::ast::Crate>)>>
  23: rustc_span::with_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-beta.1 (b955c8271 2023-03-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Z unstable-options -C incremental=[REDACTED] -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z inline-mir -C lto=off -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z panic_abort_tests -Z binary-dep-depinfo -Z force-unstable-if-unmarked

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [resolver_for_lowering] getting the resolver for lowering
end of query stack
error: could not compile `alloc`
