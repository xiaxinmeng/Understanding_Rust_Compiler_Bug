plain
    Checking clap_lex v0.2.2
error[E0023]: this pattern has 6 fields, but the corresponding tuple variant has 7 fields
    --> src/tools/clippy/clippy_utils/src/ast_utils.rs:171:18
     |
171  |         (Closure(lc, la, lm, lf, lb, _), Closure(rc, ra, rm, rf, rb, _)) => {
     |                  ^^  ^^  ^^  ^^  ^^  ^ expected 7 fields, found 6
    ::: /checkout/compiler/rustc_ast/src/ast.rs:1393:13
     |
     |
1393 |     Closure(ClosureBinder, CaptureBy, Async, Movability, P<FnDecl>, P<Expr>, Span),
     |
help: use `_` to explicitly ignore each field
     |
     |
171  |         (Closure(lc, la, lm, lf, lb, _, _), Closure(rc, ra, rm, rf, rb, _)) => {

error[E0023]: this pattern has 6 fields, but the corresponding tuple variant has 7 fields
    --> src/tools/clippy/clippy_utils/src/ast_utils.rs:171:50
     |
     |
171  |         (Closure(lc, la, lm, lf, lb, _), Closure(rc, ra, rm, rf, rb, _)) => {
     |                                                  ^^  ^^  ^^  ^^  ^^  ^ expected 7 fields, found 6
    ::: /checkout/compiler/rustc_ast/src/ast.rs:1393:13
     |
     |
1393 |     Closure(ClosureBinder, CaptureBy, Async, Movability, P<FnDecl>, P<Expr>, Span),
     |
help: use `_` to explicitly ignore each field
     |
     |
171  |         (Closure(lc, la, lm, lf, lb, _), Closure(rc, ra, rm, rf, rb, _, _)) => {

   Compiling libz-sys v1.1.3
    Checking pulldown-cmark v0.9.1
    Checking aho-corasick v0.7.18
