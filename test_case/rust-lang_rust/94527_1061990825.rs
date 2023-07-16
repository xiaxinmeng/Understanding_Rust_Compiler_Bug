plain
    Checking ar v0.8.0
    Checking gccjit_sys v0.0.1 (https://github.com/antoyo/gccjit.rs#0672b78d)
    Checking gccjit v1.0.0 (https://github.com/antoyo/gccjit.rs#0672b78d)
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error[E0599]: no variant named `Primitive` found for enum `rustc_target::abi::Scalar`
    |
    |
322 |             abi::Scalar::Primitive { value: Primitive::Pointer, valid_range: WrappingRange { start: 0, end: !0 } },
    |                          ^^^^^^^^^ variant not found in `rustc_target::abi::Scalar`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_codegen_gcc` due to previous error
Build completed unsuccessfully in 0:03:24
