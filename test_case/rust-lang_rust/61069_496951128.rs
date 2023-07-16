rust
use std::fmt::Debug;

struct D<A: Debug>(*const A);
struct Two<A: Debug> { d: D<A>, a: A }

impl<A: Debug> Drop for Two<A> {
    fn drop(&mut self) {
        println!("Dropping Two");
        self.d.0 = &mut self.a as *const A;
    }
}
    
impl<A: Debug> Drop for D<A> {
    fn drop(&mut self) {
        let a: A = unsafe { std::ptr::read(self.0) };
        println!("Dropping D({:?})", a);
        std::mem::forget(a);
    }
}

fn main() {
    let t = Two { d: D(std::ptr::null()), a: format!("Hello") };
}
