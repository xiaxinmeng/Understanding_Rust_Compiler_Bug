rust
use core::ptr::addr_of;
use core::ops::Deref;

fn main() {
    unsafe {
        struct W<T>(T);
        
        impl<T> Deref for W<T> {
            type Target = T;
            fn deref(&self) -> &T { &self.0 }
        }
        
        let w: W<i32> = W(5);
        let w = addr_of!(w);
        let p: *const i32 = addr_of!(**w); // LINT
    }
}
