plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
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
error: could not compile `rustc_expand` due to 5 previous errors
warning: build failed, waiting for other jobs to finish...
