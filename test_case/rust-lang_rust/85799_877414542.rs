plain
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error[E0004]: non-exhaustive patterns: `Unsized(_)` not covered
   --> compiler/rustc_typeck/src/astconv/mod.rs:879:19
    |
879 |             match *ast_bound {
    |                   ^^^^^^^^^^ pattern `Unsized(_)` not covered
   ::: /checkout/compiler/rustc_hir/src/hir.rs:404:5
    |
404 |     Unsized(Span),
    |     ------- not covered
    |     ------- not covered
    |
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `rustc_hir::GenericBound`

error[E0004]: non-exhaustive patterns: `&Unsized(_)` not covered
     |
2182 |                     match bound {
2182 |                     match bound {
     |                           ^^^^^ pattern `&Unsized(_)` not covered
    ::: /checkout/compiler/rustc_hir/src/hir.rs:404:5
     |
404  |     Unsized(Span),
     |     ------- not covered
     |     ------- not covered
     |
     = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
     = note: the matched value is of type `&rustc_hir::GenericBound`

error[E0004]: non-exhaustive patterns: `Unsized(_)` not covered
     |
     |
2420 |     match *bound {
     |           ^^^^^^ pattern `Unsized(_)` not covered
    ::: /checkout/compiler/rustc_hir/src/hir.rs:404:5
     |
404  |     Unsized(Span),
     |     ------- not covered
