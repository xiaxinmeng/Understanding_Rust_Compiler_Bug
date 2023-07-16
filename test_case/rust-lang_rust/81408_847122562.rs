rust
// lib.rs
use std::sync::atomic::{AtomicPtr, Ordering};

#[inline(always)]
pub fn memrchr() {
    fn detect() {}
		
    static FN: AtomicPtr<()> = AtomicPtr::new(detect as *mut ());
	
    unsafe {
        let fun = FN.load(Ordering::SeqCst);
        std::mem::transmute::<*mut (), fn()>(fun)()
    }
}	

// main.rs
fn main() {
	badlld::memrchr();
}
