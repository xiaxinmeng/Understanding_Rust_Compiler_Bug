plain
    Checking ar v0.8.0
    Checking gccjit_sys v0.0.1 (https://github.com/antoyo/gccjit.rs#0672b78d)
    Checking gccjit v1.0.0 (https://github.com/antoyo/gccjit.rs#0672b78d)
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error[E0063]: missing field `current_block` in initializer of `CodegenCx<'gcc, 'tcx>`
    |
160 |         Self {
    |         ^^^^ missing `current_block`

