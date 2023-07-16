plain
    Checking clippy_lints v0.1.66 (/checkout/src/tools/clippy/clippy_lints)
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/misc_early/mod.rs:404:39
    |
404 |             literal_suffix::check(cx, lit, &lit_snip, suffix, "integer");
    |             ---------------------     ^^^ expected `&rustc_ast::Lit`, found struct `rustc_ast::token::Lit`
    |             arguments to this function are incorrect
    |
note: function defined here
   --> src/tools/clippy/clippy_lints/src/misc_early/literal_suffix.rs:8:15
   --> src/tools/clippy/clippy_lints/src/misc_early/literal_suffix.rs:8:15
    |
8   | pub(super) fn check(cx: &EarlyContext<'_>, lit: &Lit, lit_snip: &str, suffix: &str, sugg_type: &str) {

error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/misc_early/mod.rs:406:52
    |
    |
406 |                 mixed_case_hex_literals::check(cx, lit, suffix, &lit_snip);
    |                 ------------------------------     ^^^ expected `&rustc_ast::Lit`, found struct `rustc_ast::token::Lit`
    |                 arguments to this function are incorrect
    |
note: function defined here
note: function defined here
   --> src/tools/clippy/clippy_lints/src/misc_early/mixed_case_hex_literals.rs:7:15
    |
7   | pub(super) fn check(cx: &EarlyContext<'_>, lit: &Lit, suffix: &str, lit_snip: &str) {

error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/misc_early/mod.rs:410:50
    |
    |
410 |                 zero_prefixed_literal::check(cx, lit, &lit_snip);
    |                 ----------------------------     ^^^ expected `&rustc_ast::Lit`, found struct `rustc_ast::token::Lit`
    |                 arguments to this function are incorrect
    |
note: function defined here
note: function defined here
   --> src/tools/clippy/clippy_lints/src/misc_early/zero_prefixed_literal.rs:8:15
    |
8   | pub(super) fn check(cx: &EarlyContext<'_>, lit: &Lit, lit_snip: &str) {

error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/misc_early/mod.rs:414:39
    |
    |
414 |             literal_suffix::check(cx, lit, &lit_snip, suffix, "float");
    |             ---------------------     ^^^ expected `&rustc_ast::Lit`, found struct `rustc_ast::token::Lit`
    |             arguments to this function are incorrect
    |
note: function defined here
   --> src/tools/clippy/clippy_lints/src/misc_early/literal_suffix.rs:8:15
   --> src/tools/clippy/clippy_lints/src/misc_early/literal_suffix.rs:8:15
    |
8   | pub(super) fn check(cx: &EarlyContext<'_>, lit: &Lit, lit_snip: &str, suffix: &str, sugg_type: &str) {


error[E0609]: no field `token_lit` on type `&rustc_ast::token::Lit`
  --> src/tools/clippy/clippy_lints/src/octal_escapes.rs:60:29
   |
60 |             if matches!(lit.token_lit.kind, LitKind::Str) {
   |
   |
   = note: available fields are: `kind`, `symbol`, `suffix`

error[E0609]: no field `token_lit` on type `&rustc_ast::token::Lit`
  --> src/tools/clippy/clippy_lints/src/octal_escapes.rs:61:36
   |
61 |                 check_lit(cx, &lit.token_lit, lit.span, true);
   |
   |
   = note: available fields are: `kind`, `symbol`, `suffix`

error[E0609]: no field `span` on type `&rustc_ast::token::Lit`
  --> src/tools/clippy/clippy_lints/src/octal_escapes.rs:61:51
   |
61 |                 check_lit(cx, &lit.token_lit, lit.span, true);
   |
   |
   = note: available fields are: `kind`, `symbol`, `suffix`

error[E0609]: no field `token_lit` on type `&rustc_ast::token::Lit`
  --> src/tools/clippy/clippy_lints/src/octal_escapes.rs:62:36
   |
62 |             } else if matches!(lit.token_lit.kind, LitKind::ByteStr) {
   |
   |
   = note: available fields are: `kind`, `symbol`, `suffix`

error[E0609]: no field `token_lit` on type `&rustc_ast::token::Lit`
  --> src/tools/clippy/clippy_lints/src/octal_escapes.rs:63:36
   |
63 |                 check_lit(cx, &lit.token_lit, lit.span, false);
   |
   |
   = note: available fields are: `kind`, `symbol`, `suffix`

error[E0609]: no field `span` on type `&rustc_ast::token::Lit`
  --> src/tools/clippy/clippy_lints/src/octal_escapes.rs:63:51
   |
63 |                 check_lit(cx, &lit.token_lit, lit.span, false);
   |
   |
   = note: available fields are: `kind`, `symbol`, `suffix`
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/precedence.rs:123:24
    |
    |
123 |                 if let LitKind::Int(..) | LitKind::Float(..) = &lit.kind;
    |                        ^^^^^^^^^^^^^^^^                        --------- this expression has type `&rustc_ast::token::LitKind`
    |                        expected enum `rustc_ast::token::LitKind`, found enum `rustc_ast::LitKind`

error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/precedence.rs:123:43
   --> src/tools/clippy/clippy_lints/src/precedence.rs:123:43
    |
123 |                 if let LitKind::Int(..) | LitKind::Float(..) = &lit.kind;
    |                                           ^^^^^^^^^^^^^^^^^^   --------- this expression has type `&rustc_ast::token::LitKind`
    |                                           expected enum `rustc_ast::token::LitKind`, found enum `rustc_ast::LitKind`

error[E0308]: mismatched types
  --> src/tools/clippy/clippy_lints/src/unused_rounding.rs:37:16
  --> src/tools/clippy/clippy_lints/src/unused_rounding.rs:37:16
   |
37 |         && let LitKind::Float(symbol, ty) = spanned.kind {
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^   ------------ this expression has type `rustc_ast::token::LitKind`
   |                expected enum `rustc_ast::token::LitKind`, found enum `rustc_ast::LitKind`

Some errors have detailed explanations: E0308, E0609.
For more information about an error, try `rustc --explain E0308`.
