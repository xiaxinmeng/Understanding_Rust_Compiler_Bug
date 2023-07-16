plain
    Checking cranelift-frontend v0.95.1
    Checking cranelift-native v0.95.1
    Checking cranelift-object v0.95.1
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0425]: cannot find value `dst` in this scope
    |
    |
576 |             let pointee_ty = dst.layout().ty.builtin_deref(true).unwrap().ty;

error[E0308]: mismatched types
   --> src/intrinsics/mod.rs:579:60
    |
    |
579 |                 let x_place = CPlace::for_ptr(Pointer::new(x_ptr), pointee_layout);
    |                                               ------------ ^^^^^ expected `Value`, found `CValue<'_>`
    |                                               arguments to this function are incorrect
    |
note: associated function defined here
   --> src/pointer.rs:25:19
   --> src/pointer.rs:25:19
    |
25  |     pub(crate) fn new(addr: Value) -> Self {

error[E0308]: mismatched types
   --> src/intrinsics/mod.rs:580:60
    |
    |
580 |                 let y_place = CPlace::for_ptr(Pointer::new(y_ptr), pointee_layout);
    |                                               ------------ ^^^^^ expected `Value`, found `CValue<'_>`
    |                                               arguments to this function are incorrect
    |
note: associated function defined here
   --> src/pointer.rs:25:19
   --> src/pointer.rs:25:19
    |
25  |     pub(crate) fn new(addr: Value) -> Self {


error[E0599]: no method named `load_scalar` found for struct `value_and_place::CPlace` in the current scope
    |
    |
581 |                 let x_val = x_place.load_scalar(fx);
    |                                     ^^^^^^^^^^^ method not found in `CPlace<'_>`
   ::: src/value_and_place.rs:315:1
    |
315 | pub(crate) struct CPlace<'tcx> {
315 | pub(crate) struct CPlace<'tcx> {
    | ------------------------------ method `load_scalar` not found for this struct

error[E0599]: no method named `load_scalar` found for struct `value_and_place::CPlace` in the current scope
    |
    |
582 |                 let y_val = y_place.load_scalar(fx);
    |                                     ^^^^^^^^^^^ method not found in `CPlace<'_>`
   ::: src/value_and_place.rs:315:1
    |
315 | pub(crate) struct CPlace<'tcx> {
315 | pub(crate) struct CPlace<'tcx> {
    | ------------------------------ method `load_scalar` not found for this struct
Some errors have detailed explanations: E0308, E0425, E0599.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `rustc_codegen_cranelift` (lib) due to 5 previous errors
Build completed unsuccessfully in 0:01:40
