rust
#![feature(asm, link_llvm_intrinsics)]

extern "C" {
    #[link_name = "llvm.sideeffect"]
    pub fn sideeffect();
}

#[no_mangle]
#[inline(never)]
unsafe extern "C" fn do_foo(_arg: u32) {
    sideeffect();
}

fn main() {
    unsafe { do_foo(4) };
    unsafe { do_foo(12) };
}
