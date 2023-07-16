plain
    Checking unicode-normalization v0.1.13
    Checking semver-parser v0.10.2
   Compiling clippy v0.1.57 (/checkout/src/tools/clippy)
    Checking bstr v0.2.13
error[E0532]: expected tuple struct or tuple variant, found struct `TyAliasKind`
     |
     |
269  |         (TyAlias(box TyAliasKind(ld, lg, lb, lt)), TyAlias(box TyAliasKind(rd, rg, rb, rt))) => {
     |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use struct pattern syntax instead: `TyAliasKind { def, span, params, front_where_clause, back_where_clause, bounds, ty }`
    ::: /checkout/compiler/rustc_ast/src/ast.rs:2657:1
     |
     |
2657 | pub struct TyAliasKind {
     | ---------------------- `TyAliasKind` defined here

error[E0532]: expected tuple struct or tuple variant, found struct `TyAliasKind`
     |
     |
269  |         (TyAlias(box TyAliasKind(ld, lg, lb, lt)), TyAlias(box TyAliasKind(rd, rg, rb, rt))) => {
     |                                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use struct pattern syntax instead: `TyAliasKind { def, span, params, front_where_clause, back_where_clause, bounds, ty }`
    ::: /checkout/compiler/rustc_ast/src/ast.rs:2657:1
     |
     |
2657 | pub struct TyAliasKind {
     | ---------------------- `TyAliasKind` defined here

error[E0532]: expected tuple struct or tuple variant, found struct `TyAliasKind`
     |
     |
331  |         (TyAlias(box TyAliasKind(ld, lg, lb, lt)), TyAlias(box TyAliasKind(rd, rg, rb, rt))) => {
     |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use struct pattern syntax instead: `TyAliasKind { def, span, params, front_where_clause, back_where_clause, bounds, ty }`
    ::: /checkout/compiler/rustc_ast/src/ast.rs:2657:1
     |
     |
2657 | pub struct TyAliasKind {
     | ---------------------- `TyAliasKind` defined here

error[E0532]: expected tuple struct or tuple variant, found struct `TyAliasKind`
     |
     |
331  |         (TyAlias(box TyAliasKind(ld, lg, lb, lt)), TyAlias(box TyAliasKind(rd, rg, rb, rt))) => {
     |                                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use struct pattern syntax instead: `TyAliasKind { def, span, params, front_where_clause, back_where_clause, bounds, ty }`
    ::: /checkout/compiler/rustc_ast/src/ast.rs:2657:1
     |
     |
2657 | pub struct TyAliasKind {
     | ---------------------- `TyAliasKind` defined here

error[E0532]: expected tuple struct or tuple variant, found struct `TyAliasKind`
     |
     |
349  |         (TyAlias(box TyAliasKind(ld, lg, lb, lt)), TyAlias(box TyAliasKind(rd, rg, rb, rt))) => {
     |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use struct pattern syntax instead: `TyAliasKind { def, span, params, front_where_clause, back_where_clause, bounds, ty }`
    ::: /checkout/compiler/rustc_ast/src/ast.rs:2657:1
     |
     |
2657 | pub struct TyAliasKind {
     | ---------------------- `TyAliasKind` defined here

error[E0532]: expected tuple struct or tuple variant, found struct `TyAliasKind`
     |
     |
349  |         (TyAlias(box TyAliasKind(ld, lg, lb, lt)), TyAlias(box TyAliasKind(rd, rg, rb, rt))) => {
     |                                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use struct pattern syntax instead: `TyAliasKind { def, span, params, front_where_clause, back_where_clause, bounds, ty }`
    ::: /checkout/compiler/rustc_ast/src/ast.rs:2657:1
     |
     |
2657 | pub struct TyAliasKind {
     | ---------------------- `TyAliasKind` defined here
    Checking quote v1.0.7
   Compiling libz-sys v1.1.3
    Checking getrandom v0.2.0
    Checking dirs-sys-next v0.1.2
