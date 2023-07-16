
warning: unnecessary parentheses around `match` scrutinee expression
 --> fuzz_input.rs:2:1
  |
2 | (; {`
  | ^^^^
  |
  = note: `#[warn(unused_parens)]` on by default
thread 'rustc' panicked at 'attempt to add with overflow', rust/compiler/rustc_errors/src/emitter.rs:1726:25
stack backtrace:
   0: rust_begin_unwind
             at /rustc/d3ad51b48f83329fac0cd8a9f1253f3146613c1c/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/d3ad51b48f83329fac0cd8a9f1253f3146613c1c/library/core/src/panicking.rs:143:14
   2: core::panicking::panic
             at /rustc/d3ad51b48f83329fac0cd8a9f1253f3146613c1c/library/core/src/panicking.rs:48:5
   3: rustc_errors::emitter::EmitterWriter::emit_messages_default
   4: <rustc_errors::emitter::EmitterWriter as rustc_errors::emitter::Emitter>::emit_diagnostic
   5: rustc_errors::HandlerInner::emit_diagnostic
   6: <() as rustc_errors::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
   7: core::ops::function::FnOnce::call_once{{vtable.shim}}
   8: rustc_middle::lint::struct_lint_level::struct_lint_level_impl
   9: rustc_lint::unused::UnusedDelimLint::emit_unused_delims
  10: <rustc_lint::unused::UnusedParens as rustc_lint::unused::UnusedDelimLint>::check_unused_delims_expr
  11: rustc_lint::unused::UnusedDelimLint::check_expr
  12: <rustc_lint::BuiltinCombinedEarlyLintPass as rustc_lint::passes::EarlyLintPass>::check_expr
  13: <rustc_lint::early::EarlyContextAndPass<T> as rustc_ast::visit::Visitor>::visit_expr
  14: rustc_ast::visit::walk_fn
  15: <rustc_lint::early::EarlyContextAndPass<T> as rustc_ast::visit::Visitor>::visit_item::{{closure}}
  16: rustc_interface::queries::Queries::global_ctxt
  17: rustc_interface::interface::run_compiler::{{closure}}
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
