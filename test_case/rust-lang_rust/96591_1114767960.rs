plain
   |
18 | use rustc_middle::mir::interpret::ConstValue;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: unnecessary parentheses around pattern
   --> src/librustdoc/clean/utils.rs:268:13
    |
    |
268 |             (Some(ty::ValTree::Leaf(_), _)) => {
    |             ^                             ^
    |
    = note: `-D unused-parens` implied by `-D warnings`
help: remove these parentheses
    |
268 -             (Some(ty::ValTree::Leaf(_), _)) => {
268 +             Some(ty::ValTree::Leaf(_), _) => {

error[E0308]: mismatched types
   --> src/librustdoc/clean/utils.rs:268:14
    |
    |
265 |         match (val, ty.kind()) {
    |               ---------------- this expression has type `(std::option::Option<ValTree<'_>>, &rustc_middle::ty::TyKind<'_>)`
...
268 |             (Some(ty::ValTree::Leaf(_), _)) => {
    |
    |
    = note: expected tuple `(std::option::Option<ValTree<'_>>, &rustc_middle::ty::TyKind<'_>)`

error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 1 field
   --> src/librustdoc/clean/utils.rs:268:19
    |
    |
268 |             (Some(ty::ValTree::Leaf(_), _)) => {
    |                   ^^^^^^^^^^^^^^^^^^^^  ^ expected 1 field, found 2
   ::: /checkout/library/core/src/option.rs:526:56
    |
    |
526 |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
    |                                                        - tuple variant has 1 field
error[E0308]: mismatched types
   --> src/librustdoc/clean/utils.rs:307:10
    |
    |
306 |     match (ct.val(), ct.ty().kind()) {
    |           -------------------------- this expression has type `(ConstKind<'_>, &rustc_middle::ty::TyKind<'_>)`
307 |         (ty::ValTree::Leaf(int), ty::Uint(ui)) => {
    |          ^^^^^^^^^^^^^^^^^^^^^^ expected enum `ConstKind`, found enum `ValTree`
error[E0308]: mismatched types
   --> src/librustdoc/clean/utils.rs:310:10
    |
    |
306 |     match (ct.val(), ct.ty().kind()) {
    |           -------------------------- this expression has type `(ConstKind<'_>, &rustc_middle::ty::TyKind<'_>)`
...
310 |         (ty::ValTree::Leaf(int), ty::Int(i)) => {
    |          ^^^^^^^^^^^^^^^^^^^^^^ expected enum `ConstKind`, found enum `ValTree`
Some errors have detailed explanations: E0023, E0308.
For more information about an error, try `rustc --explain E0023`.
error: could not compile `rustdoc` due to 6 previous errors
Build completed unsuccessfully in 0:02:52
