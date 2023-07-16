plain
    Checking unicode-normalization v0.1.13
   Compiling tokio v1.8.4
   Compiling clippy v0.1.60 (/checkout/src/tools/clippy)
    Checking getopts v0.2.21
error[E0050]: method `borrow` has 4 parameters but the declaration in trait `rustc_typeck::expr_use_visitor::Delegate::borrow` has 5
   --> src/tools/clippy/clippy_utils/src/sugg.rs:889:15
    |
889 |     fn borrow(&mut self, cmt: &PlaceWithHirId<'tcx>, _: HirId, _: ty::BorrowKind) {
    |
    |
    = note: `borrow` from trait: `fn(&mut Self, &PlaceWithHirId<'tcx>, HirId, rustc_middle::ty::BorrowKind, bool)`

error[E0050]: method `borrow` has 4 parameters but the declaration in trait `rustc_typeck::expr_use_visitor::Delegate::borrow` has 5
   |
   |
67 |     fn borrow(&mut self, cmt: &PlaceWithHirId<'tcx>, _: HirId, bk: ty::BorrowKind) {
   |
   |
   = note: `borrow` from trait: `fn(&mut Self, &PlaceWithHirId<'tcx>, HirId, rustc_middle::ty::BorrowKind, bool)`
   Compiling libz-sys v1.1.3
    Checking pulldown-cmark v0.9.1
    Checking aho-corasick v0.7.18
    Checking bstr v0.2.13
