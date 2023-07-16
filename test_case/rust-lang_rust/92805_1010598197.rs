plain
    Checking clippy_lints v0.1.59 (/checkout/src/tools/clippy/clippy_lints)
error: unused import: `TyCtxt`
  --> src/tools/clippy/clippy_lints/src/redundant_clone.rs:17:53
   |
17 | use rustc_middle::ty::{self, fold::TypeVisitor, Ty, TyCtxt};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
error[E0061]: this function takes 2 arguments but 3 arguments were supplied
    --> src/tools/clippy/clippy_lints/src/methods/mod.rs:2202:17
     |
     |
2202 |             if !contains_ty(cx.tcx, ret_ty, self_ty);
     |                 |
     |                 expected 2 arguments
     |
note: function defined here
note: function defined here
    --> /checkout/src/tools/clippy/clippy_utils/src/ty.rs:40:8
     |
40   | pub fn contains_ty(ty: Ty<'_>, other_ty: Ty<'_>) -> bool {

error[E0061]: this function takes 0 arguments but 1 argument was supplied
   --> src/tools/clippy/clippy_lints/src/non_send_fields_in_send_ty.rs:175:8
    |
    |
175 |     ty.walk(cx.tcx)
    |        |
    |        expected 0 arguments
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_middle/src/ty/walk.rs:110:12
    |
110 |     pub fn walk(&'tcx self) -> TypeWalker<'tcx> {

error[E0061]: this function takes 0 arguments but 1 argument was supplied
   --> src/tools/clippy/clippy_lints/src/non_send_fields_in_send_ty.rs:229:30
    |
    |
229 |     for ty_node in target_ty.walk(cx.tcx) {
    |                              |
    |                              expected 0 arguments
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_middle/src/ty/walk.rs:110:12
    |
110 |     pub fn walk(&'tcx self) -> TypeWalker<'tcx> {


error[E0618]: expected function, found `ContainsRegion`
    |
    |
627 |             if ContainsRegion(self.cx.tcx)
    |                |
    |                call expression requires function
...
706 | struct ContainsRegion;
706 | struct ContainsRegion;
    | ---------------------- `ContainsRegion` defined here
error[E0061]: this function takes 0 arguments but 1 argument was supplied
   --> src/tools/clippy/clippy_lints/src/unnecessary_sort_by.rs:229:14
    |
    |
229 |             .walk(cx.tcx)
    |              |
    |              expected 0 arguments
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_middle/src/ty/walk.rs:110:12
    |
110 |     pub fn walk(&'tcx self) -> TypeWalker<'tcx> {

Some errors have detailed explanations: E0061, E0618.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `clippy_lints` due to 6 previous errors
