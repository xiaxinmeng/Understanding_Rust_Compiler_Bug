
error: invalid `struct` delimiters or `fn` call arguments
 --> fuzz_input.rs:1:5
  |
1 |   #[B=L(E:..,B=;
  |  _____^
2 | | ,t
  | |___^
  |
help: if `L` is a struct, use braces as delimiters
  |
1 ~ #[B=L { E:..,B=;
2 ~ ,t }
  |
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
   6: <rustc_errors::ErrorReported as rustc_errors::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
   7: rustc_parse::parser::expr::<impl rustc_parse::parser::Parser>::parse_dot_or_call_expr_with_
   8: rustc_parse::parser::expr::<impl rustc_parse::parser::Parser>::collect_tokens_for_expr::{{closure}}
   9: rustc_parse::parser::expr::<impl rustc_parse::parser::Parser>::parse_prefix_expr
  10: rustc_parse::parser::expr::<impl rustc_parse::parser::Parser>::parse_assoc_expr_with
  11: rustc_parse::parser::Parser::collect_tokens_no_attrs
  12: rustc_parse::parser::Parser::parse_mac_args_common
  13: rustc_parse::parser::attr::<impl rustc_parse::parser::Parser>::parse_attr_item
  14: rustc_parse::parser::attr::<impl rustc_parse::parser::Parser>::parse_attribute
  15: rustc_parse::parser::attr::<impl rustc_parse::parser::Parser>::parse_outer_attributes
  16: rustc_parse::parser::item::<impl rustc_parse::parser::Parser>::parse_mod
  17: rustc_parse::parse_crate_from_file
  18: rustc_interface::queries::Queries::parse
  19: rustc_interface::interface::run_compiler::{{closure}}
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
