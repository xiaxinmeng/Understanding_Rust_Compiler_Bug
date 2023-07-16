plain
   Compiling rand v0.7.3
   Compiling rustc-rayon v0.3.0
   Compiling tempfile v3.1.0
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0026]: variant `rustc_mir::interpret::Scalar::Raw` does not have a field named `data`
   --> src/librustdoc/clean/utils.rs:501:64
    |
501 |         (ty::ConstKind::Value(ConstValue::Scalar(Scalar::Raw { data, .. })), ty::Uint(ui)) => {
    |                                                                ^^^^ variant `rustc_mir::interpret::Scalar::Raw` does not have this field

error[E0026]: variant `rustc_mir::interpret::Scalar::Raw` does not have a field named `data`
   --> src/librustdoc/clean/utils.rs:504:64
    |
504 |         (ty::ConstKind::Value(ConstValue::Scalar(Scalar::Raw { data, .. })), ty::Int(i)) => {
    |                                                                ^^^^ variant `rustc_mir::interpret::Scalar::Raw` does not have this field
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0026`.
error: could not compile `rustdoc`.
---
== clock drift check ==
  local time: Sat Sep 26 13:50:56 UTC 2020
  network time: Sat, 26 Sep 2020 13:50:57 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (3931) (python)
