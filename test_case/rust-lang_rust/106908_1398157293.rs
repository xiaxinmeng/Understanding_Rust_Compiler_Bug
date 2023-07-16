rust
#![feature(custom_mir, core_intrinsics)]
#![allow(unused_assignments)]
extern crate core;
use core::intrinsics::mir::*;

fn cmp_ref(a: &u8, b: &u8) -> bool {
    std::ptr::eq(a as *const u8, b as *const u8)
}

#[custom_mir(dialect = "analysis", phase = "post-cleanup")]
fn f() -> bool { mir!(
    {
        let a = 5_u8;
        let r1 = &a;
        let b = a;
        let r2 = &b;
        Call(RET, next, cmp_ref(r1, r2))
    }
    next = {
        Return()
    }
)}

fn main() {
    assert!(!f());
}
