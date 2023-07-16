plain
    Checking gccjit_sys v0.0.1 (https://github.com/antoyo/gccjit.rs#2d4fea73)
    Checking gccjit v1.0.0 (https://github.com/antoyo/gccjit.rs#2d4fea73)
    Checking object v0.25.3
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error[E0220]: associated type `RValue` not found for `Self`
    |
    |
922 |     fn typeid_metadata(&mut self, _typeid: String) -> Self::RValue {
    |                                                             ^^^^^^ help: there is an associated type with a similar name: `Value`
For more information about this error, try `rustc --explain E0220`.
error: could not compile `rustc_codegen_gcc` due to previous error
Build completed unsuccessfully in 0:03:56
