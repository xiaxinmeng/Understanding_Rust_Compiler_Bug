plain
    Checking rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
    Checking rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
error[E0252]: the name `MultiSpan` is defined multiple times
   |
   |
36 | use rustc_span::source_map::{MultiSpan, Span, DUMMY_SP};
   |                              --------- previous import of the type `MultiSpan` here
37 | use rustc_span::symbol::{kw, sym, Ident, Symbol};
38 | use rustc_span::MultiSpan;
   |     ^^^^^^^^^^^^^^^^^^^^^ `MultiSpan` reimported here
   |
   = note: `MultiSpan` must be defined only once in the type namespace of this module
error: unused import: `rustc_span::MultiSpan`
  --> compiler/rustc_parse/src/parser/mod.rs:38:5
   |
38 | use rustc_span::MultiSpan;
38 | use rustc_span::MultiSpan;
   |     ^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
    Checking chalk-engine v0.55.0
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
For more information about this error, try `rustc --explain E0252`.
error: could not compile `rustc_parse` due to 2 previous errors
