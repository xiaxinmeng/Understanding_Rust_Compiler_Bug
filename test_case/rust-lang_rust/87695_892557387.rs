plain
   Compiling rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
error[E0061]: this function takes 3 arguments but 2 arguments were supplied
   --> compiler/rustc_codegen_ssa/src/base.rs:172:24
    |
172 |                     bx.inbounds_gep(llvtable, &[bx.const_usize(u64::try_from(entry_idx).unwrap())]);
    |                        |
    |                        expected 3 arguments
    |
note: associated function defined here
