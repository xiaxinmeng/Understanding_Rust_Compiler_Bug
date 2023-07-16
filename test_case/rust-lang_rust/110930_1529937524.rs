
use std::mem::ManuallyDrop;
use std::ops::Drop;

#[inline(always)]
fn called_when_dropped() {
    let _s = "this is inlined";
}

struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        called_when_dropped()
    }
}

fn func(arg: Foo) {
    let mut md = unsafe { ManuallyDrop::new(arg) };
    unsafe { ManuallyDrop::drop(&mut md) };
}

fn main() {
    let v = Foo {};
    func(v);
}
