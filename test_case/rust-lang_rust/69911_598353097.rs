rustc
#![feature(link_llvm_intrinsics)]

extern "C" {
    #[link_name = "llvm.x86.sse2.pause"]
    fn pause();
}
