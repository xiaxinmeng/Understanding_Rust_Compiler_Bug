plain
    Checking term v0.7.0
    Checking parking_lot v0.12.1
   Compiling libz-sys v1.1.3
    Checking tester v0.9.0
error[E0609]: no field `qind` on type `&rustc_ast::Lit`
    |
    |
155 |         (Lit(l), Lit(r)) => l.qind == r.qind,
    |
    = note: available fields are: `token_lit`, `span`


error[E0609]: no field `qind` on type `&rustc_ast::Lit`
    |
    |
155 |         (Lit(l), Lit(r)) => l.qind == r.qind,
    |
    = note: available fields are: `token_lit`, `span`


error[E0609]: no field `qind` on type `&rustc_ast::Lit`
    |
    |
709 |         (Eq(_, MacArgsEq::Hir(ll)), Eq(_, MacArgsEq::Hir(rl))) => ll.qind == rl.qind,
    |
    = note: available fields are: `token_lit`, `span`


error[E0609]: no field `qind` on type `&rustc_ast::Lit`
    |
    |
709 |         (Eq(_, MacArgsEq::Hir(ll)), Eq(_, MacArgsEq::Hir(rl))) => ll.qind == rl.qind,
    |
    = note: available fields are: `token_lit`, `span`

    Checking idna v0.2.0
    Checking idna v0.2.0
error[E0609]: no field `qind` on type `&rustc_ast::Lit`
  --> src/tools/clippy/clippy_utils/src/numeric_literal.rs:50:49
   |
50 |         NumericLiteral::from_lit_kind(src, &lit.qind)
   |
   = note: available fields are: `token_lit`, `span`

For more information about this error, try `rustc --explain E0609`.
