rust
#![feature(custom_mir, core_intrinsics)]

extern crate core;
use core::intrinsics::mir::*;
use core::ptr::addr_of_mut;

#[custom_mir(dialect = "runtime", phase = "optimized")]
fn foo(x: &mut i32) -> i32 {
    mir!(
        let r1: &mut i32;
        let r2: &mut i32;
        let p1: *mut i32;
        let p2: *mut i32;

        {
            r1 = &mut *x;
            Retag(r1);
            r2 = &mut *r1;
            Retag(r2);
            p1 = addr_of_mut!(*r1);
            p2 = addr_of_mut!(*r2);

            RET = *p1;
            RET = *p2;
            Return()
        }
    )
}

fn main() {
    let mut x = 5;
    dbg!(foo(&mut x));
}
