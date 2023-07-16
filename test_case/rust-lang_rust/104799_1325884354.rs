plain
    Checking ar v0.8.0
    Checking gccjit_sys v0.0.1 (https://github.com/antoyo/gccjit.rs#bdb86fb5)
    Checking gccjit v1.0.0 (https://github.com/antoyo/gccjit.rs#bdb86fb5)
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error: expected `}`, found `,`
    |
241 |                         ..,
    |                         --^
    |                         | |
---
   |
11 | use rustc_span::Span;
   |     ^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: could not compile `rustc_codegen_gcc` due to 2 previous errors
Build completed unsuccessfully in 0:04:20
