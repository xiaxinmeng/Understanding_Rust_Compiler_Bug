plain
   Compiling rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0061]: this function takes 3 arguments but 2 arguments were supplied
   --> compiler/rustc_codegen_llvm/src/intrinsic.rs:325:38
    |
325 |                     let a_val = self.load(a_ptr, layout.align.abi);
    |                                      |
    |                                      expected 3 arguments
    |
note: associated function defined here
note: associated function defined here

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
   --> compiler/rustc_codegen_llvm/src/intrinsic.rs:327:38
    |
327 |                     let b_val = self.load(b_ptr, layout.align.abi);
    |                                      |
    |                                      expected 3 arguments
    |
note: associated function defined here
