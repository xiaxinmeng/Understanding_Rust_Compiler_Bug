
#![feature(lang_items)]
#![feature(no_core)]
#![feature(link_llvm_intrinsics)]
#![crate_type = "lib"]
#![no_core]

#[lang = "copy"]
trait Copy {}

#[lang = "freeze"]
trait Freeze {}

#[lang = "sized"]
trait Sized {}

#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.nvvm.barrier0"]
    fn syncthreads() -> ();
}

#[no_mangle]
pub unsafe fn foo() {
    syncthreads();
}
