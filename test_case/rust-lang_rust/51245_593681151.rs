rust
#![feature(alloc_error_hook)]

use std::alloc::set_alloc_error_hook;
use std::alloc::Layout;

fn oom(layout: Layout) {
    panic!("oom {}", layout.size());
}

fn main() {
    set_alloc_error_hook(oom);
}
