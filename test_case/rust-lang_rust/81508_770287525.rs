
% RUST_BACKTRACE=1 rustc +dev f5.rs -Ztreat-err-as-bug
error[E0433]: failed to resolve: use of undeclared type `Foo`
 --> f5.rs:7:24
  |
7 |         println!("{}", Foo::Bar);
  |                        ^^^ use of undeclared type `Foo`

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:990:27
stack backtrace:
   0: std::panicking::begin_panic
   1: rustc_errors::HandlerInner::emit_diagnostic
   2: rustc_errors::Handler::emit_diagnostic
   3: rustc_errors::diagnostic_builder::DiagnosticBuilder::emit
   4: rustc_resolve::diagnostics::<impl rustc_resolve::Resolver>::report_error
   5: rustc_resolve::late::LateResolutionVisitor::report_error
   6: rustc_resolve::late::LateResolutionVisitor::smart_resolve_path_fragment
   7: rustc_resolve::late::LateResolutionVisitor::resolve_expr
   8: rustc_resolve::late::LateResolutionVisitor::resolve_expr
   9: rustc_ast::visit::walk_expr
  10: rustc_resolve::late::LateResolutionVisitor::resolve_expr
  11: rustc_ast::visit::walk_expr
  12: rustc_resolve::late::LateResolutionVisitor::resolve_expr
  13: rustc_resolve::late::LateResolutionVisitor::resolve_expr
  14: rustc_resolve::late::LateResolutionVisitor::resolve_expr
  15: rustc_resolve::late::LateResolutionVisitor::resolve_expr
  16: <rustc_resolve::late::LateResolutionVisitor as rustc_ast::visit::Visitor>::visit_block
  17: rustc_resolve::late::LateResolutionVisitor::resolve_expr
  18: <rustc_resolve::late::LateResolutionVisitor as rustc_ast::visit::Visitor>::visit_block
  19: rustc_resolve::late::LateResolutionVisitor::with_rib
  20: <rustc_resolve::late::LateResolutionVisitor as rustc_ast::visit::Visitor>::visit_fn
  21: rustc_ast::visit::walk_item
  22: rustc_resolve::late::LateResolutionVisitor::with_generic_param_rib
  23: <rustc_resolve::late::LateResolutionVisitor as rustc_ast::visit::Visitor>::visit_item
  24: rustc_ast::visit::walk_item
  25: rustc_resolve::late::LateResolutionVisitor::with_scope
  26: <rustc_resolve::late::LateResolutionVisitor as rustc_ast::visit::Visitor>::visit_item
  27: rustc_ast::visit::walk_crate
  28: rustc_resolve::late::<impl rustc_resolve::Resolver>::late_resolve_crate
  29: rustc_session::utils::<impl rustc_session::session::Session>::time
  30: rustc_resolve::Resolver::resolve_crate
  31: rustc_interface::passes::configure_and_expand_inner
  32: rustc_interface::passes::configure_and_expand::{{closure}}
  33: alloc::boxed::<impl core::ops::generator::Generator<R> for core::pin::Pin<alloc::boxed::Box<G,A>>>::resume
  34: rustc_data_structures::box_region::PinnedGenerator<I,A,R>::new
  35: rustc_interface::passes::BoxedResolver::new
  36: rustc_interface::passes::configure_and_expand
  37: rustc_interface::queries::Query<T>::compute
  38: rustc_interface::queries::Queries::expansion
  39: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  40: rustc_span::with_source_map
  41: rustc_interface::interface::create_compiler_and_run
  42: scoped_tls::ScopedKey<T>::set
  43: rustc_span::with_session_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-dev running on x86_64-apple-darwin

note: compiler flags: -Z treat-err-as-bug

query stack during panic:
end of query stack
