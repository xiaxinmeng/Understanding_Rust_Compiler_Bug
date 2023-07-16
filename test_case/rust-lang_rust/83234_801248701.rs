plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: mismatched types
   --> src/librustdoc/clean/utils.rs:334:60
    |
334 |                 let const_ = ty::Const::from_value(cx.tcx, val, ty);
    |                                                            ^^^ expected enum `ValTree`, found enum `ConstValue`
error[E0308]: mismatched types
   --> src/librustdoc/clean/utils.rs:356:31
    |
    |
355 |     match (ct.val, ct.ty.kind()) {
    |           ---------------------- this expression has type `(ConstKind<'_>, &rustc_middle::ty::TyKind<'_>)`
356 |         (ty::ConstKind::Value(ConstValue::Scalar(int)), ty::Uint(ui)) => {
    |                               ^^^^^^^^^^^^^^^^^^^^^^^ expected enum `ValTree`, found enum `ConstValue`
error[E0308]: mismatched types
   --> src/librustdoc/clean/utils.rs:359:31
    |
    |
355 |     match (ct.val, ct.ty.kind()) {
    |           ---------------------- this expression has type `(ConstKind<'_>, &rustc_middle::ty::TyKind<'_>)`
...
359 |         (ty::ConstKind::Value(ConstValue::Scalar(int)), ty::Int(i)) => {
    |                               ^^^^^^^^^^^^^^^^^^^^^^^ expected enum `ValTree`, found enum `ConstValue`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustdoc`
