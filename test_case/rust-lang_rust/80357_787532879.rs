plain
    Checking clippy_lints v0.1.52 (/checkout/src/tools/clippy/clippy_lints)
error: mismatched closing delimiter: `}`
  --> src/tools/clippy/clippy_lints/src/if_let_some_result.rs:70:9
   |
43 |         if_chain! { //begin checking variables
   |                   - closing delimiter possibly meant for this
44 |             if let ExprKind::Let(_, ref expr, LetSource::IfLet = expr.kind; //test if expr is if let
...
70 |         }
   |         ^ mismatched closing delimiter


error: unexpected end of macro invocation
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/if_chain-1.0.0/src/lib.rs:176:44
    |
176 |         __if_chain! { @init ($($tt)* $head) $($tail)* }
    |                                            ^ missing tokens in macro arguments

error: unused imports: `is_type_diagnostic_item`, `method_chain_args`, `snippet_with_applicability`, `span_lint_and_sugg`
 --> src/tools/clippy/clippy_lints/src/if_let_some_result.rs:1:20
  |
1 | use crate::utils::{is_type_diagnostic_item, method_chain_args, snippet_with_applicability, span_lint_and_sugg};
  |                    ^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `rustc_errors::Applicability`
 --> src/tools/clippy/clippy_lints/src/if_let_some_result.rs:3:5
  |
3 | use rustc_errors::Applicability;
3 | use rustc_errors::Applicability;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: unused imports: `ExprKind`, `LetSource`, `PatKind`, `QPath`
 --> src/tools/clippy/clippy_lints/src/if_let_some_result.rs:4:23
  |
4 | use rustc_hir::{Expr, LetSource, ExprKind, PatKind, QPath};
  |                       ^^^^^^^^^  ^^^^^^^^  ^^^^^^^  ^^^^^
error: unused import: `rustc_span::sym`
 --> src/tools/clippy/clippy_lints/src/if_let_some_result.rs:7:5
  |
7 | use rustc_span::sym;
7 | use rustc_span::sym;
  |     ^^^^^^^^^^^^^^^

error[E0599]: no variant or associated item named `IfLetGuardDesugar` found for enum `MatchSource` in the current scope
    |
    |
736 |         hir::MatchSource::IfLetGuardDesugar => "MatchSource::IfLetGuardDesugar".to_string(),
    |                           ^^^^^^^^^^^^^^^^^ variant or associated item not found in `MatchSource`

error[E0599]: no variant named `IfLetDesugar` found for enum `MatchSource`
     |
     |
2000 |                 ref match_expr, ref match_arms, MatchSource::IfLetDesugar{ contains_else_clause: false }
     |                                                              ^^^^^^^^^^^^ help: there is a variant with a similar name: `AwaitDesugar`

error[E0599]: no variant named `IfLetDesugar` found for enum `MatchSource`
    |
    |
214 |             MatchSource::IfLetDesugar {
    |                          ^^^^^^^^^^^^ help: there is a variant with a similar name: `AwaitDesugar`
error: aborting due to 9 previous errors

For more information about this error, try `rustc --explain E0599`.
error: could not compile `clippy_lints`
