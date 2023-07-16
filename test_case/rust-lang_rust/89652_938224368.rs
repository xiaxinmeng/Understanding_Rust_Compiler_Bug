plain
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error[E0412]: cannot find type `Value` in this scope
   --> src/builder.rs:918:49
    |
918 |     fn type_metadata(&mut self, _function: &'ll Value, _typeid: String) {
    |
    |
   ::: /cargo/git/checkouts/gccjit.rs-13c2e290f2fb9e4d/2d4fea7/src/lvalue.rs:43:1
    |
43  | pub struct LValue<'ctx> {
    | ----------------------- similarly named struct `LValue` defined here
    |
help: there is an enum variant `rustc_ast::CaptureBy::Value` and 4 others; try using the variant's enum
    |
918 |     fn type_metadata(&mut self, _function: &'ll rustc_ast::CaptureBy, _typeid: String) {
    |                                                 ~~~~~~~~~~~~~~~~~~~~
918 |     fn type_metadata(&mut self, _function: &'ll rustc_ast::SelfKind, _typeid: String) {
    |                                                 ~~~~~~~~~~~~~~~~~~~
918 |     fn type_metadata(&mut self, _function: &'ll rustc_hir::CaptureBy, _typeid: String) {
    |                                                 ~~~~~~~~~~~~~~~~~~~~
918 |     fn type_metadata(&mut self, _function: &'ll rustc_middle::ty::ConstKind, _typeid: String) {
help: a struct with a similar name exists
    |
    |
918 |     fn type_metadata(&mut self, _function: &'ll LValue, _typeid: String) {


error[E0261]: use of undeclared lifetime name `'ll`
    |
    |
918 |     fn type_metadata(&mut self, _function: &'ll Value, _typeid: String) {
    |                                             ^^^ undeclared lifetime
    |
    = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
help: consider introducing lifetime `'ll` here
    |
385 | impl<'ll, 'a, 'gcc, 'tcx> BuilderMethods<'a, 'tcx> for Builder<'a, 'gcc, 'tcx> {
    |      ++++
help: consider introducing lifetime `'ll` here
    |
918 |     fn type_metadata<'ll>(&mut self, _function: &'ll Value, _typeid: String) {

error[E0046]: not all trait items implemented, missing: `type_test`
  --> src/intrinsic/mod.rs:76:1
   |
   |
76 | impl<'a, 'gcc, 'tcx> IntrinsicCallMethods<'tcx> for Builder<'a, 'gcc, 'tcx> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `type_test` in implementation
   |
   = help: implement the missing item: `fn type_test(&mut self, _: <Self as BackendTypes>::Value, _: <Self as BackendTypes>::Value) -> <Self as BackendTypes>::Value { todo!() }`
error[E0308]: mismatched types
   --> src/builder.rs:922:55
    |
    |
922 |     fn typeid_metadata(&mut self, _typeid: String) -> Self::Value {
    |        ---------------                                ^^^^^^^^^^^ expected struct `RValue`, found `()`
    |        |
    |        implicitly returns `()` as its body has no tail or `return` expression
Some errors have detailed explanations: E0046, E0261, E0308, E0412.
For more information about an error, try `rustc --explain E0046`.
error: could not compile `rustc_codegen_gcc` due to 4 previous errors
Build completed unsuccessfully in 0:03:19
