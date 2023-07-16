plain
    Checking url v2.1.1
    Checking toml v0.5.7
    Checking cargo_metadata v0.12.0
    Checking clippy_lints v0.1.51 (/checkout/src/tools/clippy/clippy_lints)
error[E0050]: method `borrow` has 4 parameters but the declaration in trait `rustc_typeck::expr_use_visitor::Delegate::borrow` has 5
   |
   |
73 |     fn borrow(&mut self, cmt: &PlaceWithHirId<'tcx>, _: HirId, bk: ty::BorrowKind) {
   |
   |
   = note: `borrow` from trait: `fn(&mut Self, &PlaceWithHirId<'tcx>, HirId, std::option::Option<&rustc_hir::Expr<'_>>, rustc_middle::ty::BorrowKind)`

error[E0050]: method `mutate` has 3 parameters but the declaration in trait `mutate` has 4
   |
   |
79 |     fn mutate(&mut self, cmt: &PlaceWithHirId<'tcx>, _: HirId) {
   |
   |
   = note: `mutate` from trait: `fn(&mut Self, &PlaceWithHirId<'tcx>, HirId, std::option::Option<&rustc_hir::Expr<'_>>)`

error[E0050]: method `borrow` has 4 parameters but the declaration in trait `rustc_typeck::expr_use_visitor::Delegate::borrow` has 5
   --> src/tools/clippy/clippy_lints/src/escape.rs:138:15
    |
138 |     fn borrow(&mut self, cmt: &PlaceWithHirId<'tcx>, _: HirId, _: ty::BorrowKind) {
    |
    |
    = note: `borrow` from trait: `fn(&mut Self, &PlaceWithHirId<'tcx>, HirId, std::option::Option<&rustc_hir::Expr<'_>>, rustc_middle::ty::BorrowKind)`

error[E0050]: method `mutate` has 3 parameters but the declaration in trait `mutate` has 4
   --> src/tools/clippy/clippy_lints/src/escape.rs:146:15
    |
146 |     fn mutate(&mut self, cmt: &PlaceWithHirId<'tcx>, _: HirId) {
    |
    |
    = note: `mutate` from trait: `fn(&mut Self, &PlaceWithHirId<'tcx>, HirId, std::option::Option<&rustc_hir::Expr<'_>>)`

error[E0050]: method `borrow` has 4 parameters but the declaration in trait `rustc_typeck::expr_use_visitor::Delegate::borrow` has 5
    --> src/tools/clippy/clippy_lints/src/loops.rs:1960:15
     |
1960 |     fn borrow(&mut self, cmt: &PlaceWithHirId<'tcx>, diag_expr_id: HirId, bk: ty::BorrowKind) {
     |
     |
     = note: `borrow` from trait: `fn(&mut Self, &PlaceWithHirId<'tcx>, HirId, std::option::Option<&rustc_hir::Expr<'_>>, rustc_middle::ty::BorrowKind)`

error[E0050]: method `mutate` has 3 parameters but the declaration in trait `mutate` has 4
    --> src/tools/clippy/clippy_lints/src/loops.rs:1973:15
     |
1973 |     fn mutate(&mut self, cmt: &PlaceWithHirId<'tcx>, diag_expr_id: HirId) {
     |
     |
     = note: `mutate` from trait: `fn(&mut Self, &PlaceWithHirId<'tcx>, HirId, std::option::Option<&rustc_hir::Expr<'_>>)`

error[E0050]: method `borrow` has 4 parameters but the declaration in trait `rustc_typeck::expr_use_visitor::Delegate::borrow` has 5
   --> src/tools/clippy/clippy_lints/src/needless_pass_by_value.rs:335:15
    |
335 |     fn borrow(&mut self, _: &euv::PlaceWithHirId<'tcx>, _: HirId, _: ty::BorrowKind) {}
    |
    |
    = note: `borrow` from trait: `fn(&mut Self, &PlaceWithHirId<'tcx>, HirId, std::option::Option<&rustc_hir::Expr<'_>>, rustc_middle::ty::BorrowKind)`

error[E0050]: method `mutate` has 3 parameters but the declaration in trait `mutate` has 4
   --> src/tools/clippy/clippy_lints/src/needless_pass_by_value.rs:337:15
    |
337 |     fn mutate(&mut self, _: &euv::PlaceWithHirId<'tcx>, _: HirId) {}
    |
    |
    = note: `mutate` from trait: `fn(&mut Self, &PlaceWithHirId<'tcx>, HirId, std::option::Option<&rustc_hir::Expr<'_>>)`
error[E0061]: this function takes 2 arguments but 1 argument was supplied
  --> src/tools/clippy/clippy_lints/src/utils/usage.rs:30:10
   |
   |
30 |         .walk_expr(expr);
   |          |
   |          expected 2 arguments

error[E0061]: this function takes 2 arguments but 1 argument was supplied
error[E0061]: this function takes 2 arguments but 1 argument was supplied
    --> src/tools/clippy/clippy_lints/src/loops.rs:2059:10
     |
2059 |         .walk_expr(body);
     |          |
     |          expected 2 arguments

error: aborting due to 10 previous errors
