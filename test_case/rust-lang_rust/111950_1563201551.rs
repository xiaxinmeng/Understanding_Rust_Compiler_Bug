plain
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error: variable does not need to be mutable
   --> src/common.rs:416:75
    |
416 |         let span_to_caller_location = |fx: &mut FunctionCx<'_, '_, 'tcx>, mut span: Span| {
    |                                                                           |
    |                                                                           help: remove this `mut`
    |
    |
    = note: `-D unused-mut` implied by `-D warnings`
error: could not compile `rustc_codegen_cranelift` (lib) due to previous error
Build completed unsuccessfully in 0:01:39
