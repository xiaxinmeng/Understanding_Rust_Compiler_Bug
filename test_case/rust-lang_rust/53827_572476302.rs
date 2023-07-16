
use std::alloc::{alloc, Layout}
let layout = Layout::new::<[u8; 0x7ffff000]>();
let mut buf = unsafe {
    let ptr = alloc(layout) as *mut [u8; 0x7ffff000];
    Box::from_raw(ptr)
};
