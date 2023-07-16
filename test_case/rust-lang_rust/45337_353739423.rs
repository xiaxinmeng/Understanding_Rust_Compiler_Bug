Rust
#![feature(generators, generator_trait)]

use std::cell::RefCell;
use std::ops::Generator;

fn static_alloc<'a, T>(t: T) -> &'a mut T {
    // this is an unsafe function, but it is sound (not running destructors
    // is safe), can be implemented using a static dropless arena of
    // `ManuallyDrop` data, and was planned to be added to the standard
    // library at some point.
    unsafe {
        use std::mem;
        let mut b = Box::new(t);
        let r = &mut *(&mut *b as *mut T);
        mem::forget(b);
        r
    }
}

fn main() {
    let (cell, mut gen);
    cell = Box::new(RefCell::new(0));
    let ref_ = static_alloc(Some(cell.borrow_mut()));
    // the upvar is the non-dropck `&mut Option<Ref<'a, i32>>`.
    gen = || {
        // but the generator can use it to drop a `Ref<'a, i32>`.
        let _d = ref_.take();
        yield;
    };
    gen.resume();
    // drops the RefCell and then the Ref, leading to use-after-free
}
