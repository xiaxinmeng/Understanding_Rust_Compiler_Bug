plain
[RUSTC-TIMING] rustc_errors test:false 5.909
   Compiling rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
[RUSTC-TIMING] rustc_attr test:false 3.552
   Compiling rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
error[E0432]: unresolved import `crate::mbe::MetaVarExpr`
 --> compiler/rustc_expand/src/mbe/transcribe.rs:3:24
  |
3 | use crate::mbe::{self, MetaVarExpr};
  |                        ^^^^^^^^^^^ no `MetaVarExpr` in `mbe`
error: unused import: `TokenKind`
error: unused import: `TokenKind`
 --> compiler/rustc_expand/src/mbe/transcribe.rs:5:43
  |
5 | use rustc_ast::token::{self, NtTT, Token, TokenKind};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`

error: unused imports: `DiagnosticBuilder`, `ErrorGuaranteed`
  --> compiler/rustc_expand/src/mbe/transcribe.rs:10:20
   |
10 | use rustc_errors::{DiagnosticBuilder, ErrorGuaranteed};


error: unused imports: `Ident`, `sym`
  --> compiler/rustc_expand/src/mbe/transcribe.rs:12:26
   |
12 | use rustc_span::symbol::{sym, Ident, MacroRulesNormalizedIdent};

[RUSTC-TIMING] rustc_query_system test:false 4.651
[RUSTC-TIMING] rustc_query_system test:false 4.651
error[E0599]: no variant or associated item named `MetaVarExpr` found for enum `mbe::TokenTree` in the current scope
   --> compiler/rustc_expand/src/mbe/macro_parser.rs:604:53
    |
604 |                 TokenTree::MetaVar(..) | TokenTree::MetaVarExpr(..) => unreachable!(),
    |                                                     ^^^^^^^^^^^ variant or associated item not found in `mbe::TokenTree`
    |
   ::: compiler/rustc_expand/src/mbe.rs:79:1
    |
79  | enum TokenTree {
    | -------------- variant or associated item `MetaVarExpr` not found here
Some errors have detailed explanations: E0432, E0599.
For more information about an error, try `rustc --explain E0432`.
[RUSTC-TIMING] rustc_expand test:false 0.818
error: could not compile `rustc_expand` due to 5 previous errors
