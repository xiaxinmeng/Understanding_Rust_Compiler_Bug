rust
#![feature(rustc_private)]

extern crate rustc_codegen_llvm;
extern crate rustc_driver;

fn main() {
    unsafe {
        rustc_codegen_llvm::llvm::LLVMRustContextCreate(true);
    }
}
