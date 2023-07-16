plain
    Checking gccjit_sys v0.0.1 (https://github.com/antoyo/gccjit.rs#2d4fea73)
    Checking gccjit v1.0.0 (https://github.com/antoyo/gccjit.rs#2d4fea73)
    Checking object v0.25.3
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error: hidden lifetime parameters in types are deprecated
    |
    |
922 |     fn typeid_metadata(&mut self, _typeid: String) -> RValue {
    |                                                       ^^^^^^- help: indicate the anonymous lifetime: `<'_>`
    |
    = note: `-D elided-lifetimes-in-paths` implied by `-D warnings`

error[E0495]: cannot infer an appropriate lifetime for lifetime parameter in generic type due to conflicting requirements
    |
    |
922 |     fn typeid_metadata(&mut self, _typeid: String) -> RValue {
    |
    |
note: first, the lifetime cannot outlive the anonymous lifetime defined on the method body at 922:24...
    |
    |
922 |     fn typeid_metadata(&mut self, _typeid: String) -> RValue {
    |                        ^^^^^^^^^
note: ...so that the method type is compatible with trait
    |
    |
922 |     fn typeid_metadata(&mut self, _typeid: String) -> RValue {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: expected `fn(&mut builder::Builder<'a, 'gcc, 'tcx>, std::string::String) -> RValue<'_>`
               found `fn(&mut builder::Builder<'a, 'gcc, 'tcx>, std::string::String) -> RValue<'_>`
note: but, the lifetime must be valid for the lifetime `'gcc` as defined on the impl at 385:10...
    |
    |
385 | impl<'a, 'gcc, 'tcx> BuilderMethods<'a, 'tcx> for Builder<'a, 'gcc, 'tcx> {
    |          ^^^^
note: ...so that the types are compatible
    |
    |
922 |     fn typeid_metadata(&mut self, _typeid: String) -> RValue {
    = note: expected `BackendTypes`
               found `BackendTypes`

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> src/builder.rs:922:55
    |
922 |     fn typeid_metadata(&mut self, _typeid: String) -> RValue {
    |        ---------------                                ^^^^^^ expected struct `RValue`, found `()`
    |        |
    |        implicitly returns `()` as its body has no tail or `return` expression
error[E0308]: mismatched types
   --> src/intrinsic/mod.rs:370:77
    |
    |
370 |     fn type_test(&mut self, _pointer: Self::Value, _typeid: Self::Value) -> Self::Value {
    |        ---------                                                            ^^^^^^^^^^^ expected struct `RValue`, found `()`
    |        |
    |        implicitly returns `()` as its body has no tail or `return` expression
Some errors have detailed explanations: E0308, E0495.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `rustc_codegen_gcc` due to 4 previous errors
Build completed unsuccessfully in 0:03:45
