plain
    Checking gccjit_sys v0.0.1 (https://github.com/antoyo/gccjit.rs#2d4fea73)
    Checking gccjit v1.0.0 (https://github.com/antoyo/gccjit.rs#2d4fea73)
    Checking object v0.25.3
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error: hidden lifetime parameters in types are deprecated
    |
    |
121 | ...eAsmOptions, _span: &[Span], instance: Instance) {
    |                                           ^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
    |
    = note: `-D elided-lifetimes-in-paths` implied by `-D warnings`
error: could not compile `rustc_codegen_gcc` due to previous error
Build completed unsuccessfully in 0:03:18
