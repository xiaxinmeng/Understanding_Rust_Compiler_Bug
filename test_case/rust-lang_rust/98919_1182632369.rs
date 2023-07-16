rust
use std::alloc::{alloc, dealloc, Layout};

unsafe {
    let layout = Layout::new::<u16>();
    let ptr = alloc(layout);
    assert!(!ptr.is_null());
    let val = *ptr; // Is this line UB?
    assert!(val == 0 || val > 0); // If no, is this line UB? If no, can the assertion fail?
}
