plain
    Checking ar v0.8.0
    Checking gccjit_sys v0.0.1 (https://github.com/antoyo/gccjit.rs#0672b78d)
    Checking gccjit v1.0.0 (https://github.com/antoyo/gccjit.rs#0672b78d)
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error: hidden lifetime parameters in types are deprecated
    |
    |
341 | ...lt<(RValue<'gcc>, &'tcx Allocation), ErrorHandled> {
    |                                         ^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
    |
    = note: `-D elided-lifetimes-in-paths` implied by `-D warnings`
error[E0106]: missing lifetime specifier
   --> src/consts.rs:341:134
    |
    |
341 | ...gcc, 'tcx>(cx: &CodegenCx<'gcc, 'tcx>, def_id: DefId) -> Result<(RValue<'gcc>, &'tcx Allocation), ErrorHandled> {
    |                   ----------------------                                                             ^^^^^^^^^^^^ expected named lifetime parameter
    |
    = help: this function's return type contains a borrowed value, but the signature does not say which one of `cx`'s 3 lifetimes it is borrowed from
note: these named lifetimes are available to use
    |
    |
341 | pub fn codegen_static_initializer<'gcc, 'tcx>(cx: &CodegenCx<'gcc, 'tcx>, def_id: DefId) -> Result<(RValue<'gcc>, &'tcx Allocation), Erro...

For more information about this error, try `rustc --explain E0106`.
error: could not compile `rustc_codegen_gcc` due to 2 previous errors
Build completed unsuccessfully in 0:02:58
