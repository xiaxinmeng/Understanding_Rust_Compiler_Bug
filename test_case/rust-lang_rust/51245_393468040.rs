rust
#![feature(allocator_api)]

use std::alloc::set_oom_hook;
use std::alloc::Layout;

fn oom(layout: Layout) -> ! {
    panic!("oom {}", layout.size());
}

fn main() {
    set_oom_hook(oom);
    let result = std::panic::catch_unwind(|| {
        let v = Vec::<u8>::with_capacity(100000000000000);
        println!("{:p}", &v[..]);
    });
    println!("{:?}", result);
}
