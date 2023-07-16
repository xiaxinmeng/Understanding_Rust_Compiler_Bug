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
   --> src/intrinsics/mod.rs:579:47
    |
    |
579 |                 let x_place = CPlace::for_ptr(x_ptr, pointee_layout);
    |                               --------------- ^^^^^ expected `Pointer`, found `CValue<'_>`
    |                               arguments to this function are incorrect
    |
note: associated function defined here
   --> src/value_and_place.rs:386:19
   --> src/value_and_place.rs:386:19
    |
386 |     pub(crate) fn for_ptr(ptr: Pointer, layout: TyAndLayout<'tcx>) -> CPlace<'tcx> {

error[E0308]: mismatched types
   --> src/intrinsics/mod.rs:580:47
    |
    |
580 |                 let y_place = CPlace::for_ptr(y_ptr, pointee_layout);
    |                               --------------- ^^^^^ expected `Pointer`, found `CValue<'_>`
    |                               arguments to this function are incorrect
    |
note: associated function defined here
   --> src/value_and_place.rs:386:19
   --> src/value_and_place.rs:386:19
    |
386 |     pub(crate) fn for_ptr(ptr: Pointer, layout: TyAndLayout<'tcx>) -> CPlace<'tcx> {


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
error[E0061]: this method takes 2 arguments but 1 argument was supplied
   --> src/intrinsics/mod.rs:583:25
    |
    |
583 |                 x_place.write_cvalue(y_val);
    |                         ^^^^^^^^^^^^------- an argument of type `value_and_place::CValue<'_>` is missing
note: method defined here
   --> src/value_and_place.rs:476:19
    |
    |
476 |     pub(crate) fn write_cvalue(self, fx: &mut FunctionCx<'_, '_, 'tcx>, from: CValue<'tcx>) {
help: provide the argument
    |
    |
583 |                 x_place.write_cvalue(y_val, /* value_and_place::CValue<'_> */);

error[E0061]: this method takes 2 arguments but 1 argument was supplied
   --> src/intrinsics/mod.rs:584:25
    |
    |
584 |                 y_place.write_cvalue(x_val);
    |                         ^^^^^^^^^^^^------- an argument of type `value_and_place::CValue<'_>` is missing
note: method defined here
   --> src/value_and_place.rs:476:19
    |
    |
476 |     pub(crate) fn write_cvalue(self, fx: &mut FunctionCx<'_, '_, 'tcx>, from: CValue<'tcx>) {
help: provide the argument
    |
    |
584 |                 y_place.write_cvalue(x_val, /* value_and_place::CValue<'_> */);

Some errors have detailed explanations: E0061, E0308, E0425, E0599.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `rustc_codegen_cranelift` (lib) due to 7 previous errors
