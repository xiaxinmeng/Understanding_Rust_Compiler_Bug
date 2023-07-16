plain
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error: unused variable: `instance`
   --> src/asm.rs:121:169
    |
121 | ...smOptions, _span: &[Span], instance: Instance<'_>) {
    |                               ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_instance`
    |
    = note: `-D unused-variables` implied by `-D warnings`
error: could not compile `rustc_codegen_gcc` due to previous error
Build completed unsuccessfully in 0:04:00
