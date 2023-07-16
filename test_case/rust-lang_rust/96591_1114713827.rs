plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: mismatched types
   --> src/librustdoc/clean/utils.rs:267:14
    |
265 |         match (val, ty.kind()) {
    |               ---------------- this expression has type `(std::option::Option<ValTree<'_>>, &rustc_middle::ty::TyKind<'_>)`
266 |             (_, &ty::Ref(..)) => None,
267 |             (ty::ValTree::Branch(_), &ty::Adt(_, _)) => None,
    |              ^^^^^^^^^^^^^^^^^^^^^^ expected enum `std::option::Option`, found enum `ValTree`
    |
    = note: expected enum `std::option::Option<ValTree<'_>>`
               found enum `ValTree<'_>`
error[E0308]: mismatched types
   --> src/librustdoc/clean/utils.rs:268:14
    |
265 |         match (val, ty.kind()) {
265 |         match (val, ty.kind()) {
    |               ---------------- this expression has type `(std::option::Option<ValTree<'_>>, &rustc_middle::ty::TyKind<'_>)`
...
268 |             (ty::ValTree::Leaf(_), _) => {
    |              ^^^^^^^^^^^^^^^^^^^^ expected enum `std::option::Option`, found enum `ValTree`
    |
    = note: expected enum `std::option::Option<ValTree<'_>>`
               found enum `ValTree<'_>`
error[E0308]: mismatched types
   --> src/librustdoc/clean/utils.rs:269:57
    |
    |
269 |                 let const_ = ty::Const::from_value(tcx, val, ty);
    |                                                         ^^^ expected enum `ValTree`, found enum `std::option::Option`
    |
    = note: expected enum `ValTree<'_>`
               found enum `std::option::Option<ValTree<'_>>`
error[E0308]: mismatched types
   --> src/librustdoc/clean/utils.rs:307:31
    |
    |
306 |     match (ct.val(), ct.ty().kind()) {
    |           -------------------------- this expression has type `(ConstKind<'_>, &rustc_middle::ty::TyKind<'_>)`
307 |         (ty::ConstKind::Value(ConstValue::Scalar(int)), ty::Uint(ui)) => {
    |                               ^^^^^^^^^^^^^^^^^^^^^^^ expected enum `ValTree`, found enum `ConstValue`
error[E0308]: mismatched types
   --> src/librustdoc/clean/utils.rs:310:31
    |
    |
306 |     match (ct.val(), ct.ty().kind()) {
    |           -------------------------- this expression has type `(ConstKind<'_>, &rustc_middle::ty::TyKind<'_>)`
...
310 |         (ty::ConstKind::Value(ConstValue::Scalar(int)), ty::Int(i)) => {
    |                               ^^^^^^^^^^^^^^^^^^^^^^^ expected enum `ValTree`, found enum `ConstValue`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustdoc` due to 5 previous errors
Build completed unsuccessfully in 0:03:22
