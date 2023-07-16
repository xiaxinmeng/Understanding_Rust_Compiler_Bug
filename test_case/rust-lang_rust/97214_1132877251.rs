plain
    Checking ar v0.8.0
    Checking gccjit_sys v0.0.1 (https://github.com/antoyo/gccjit.rs#bdecdecf)
    Checking gccjit v1.0.0 (https://github.com/antoyo/gccjit.rs#bdecdecf)
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error: 1st rule of macro `emit_error` is never used
   |
   |
17 |         ($msg: tt) => {
   |
   |
   = note: `-D unused-macro-rules` implied by `-D warnings`
error: could not compile `rustc_codegen_gcc` due to previous error
Build completed unsuccessfully in 0:03:49
