rust
#![feature(allocator_api)]

use std::alloc::set_oom_hook;
use std::alloc::Layout;

struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo");
    }
}

fn oom(layout: Layout) -> ! {
    let f = Foo;
    panic!("oom {}", layout.size());
}

fn main() {
    set_oom_hook(oom);
    let result = std::panic::catch_unwind(|| {
        let f = Foo;
        std::alloc::oom(Layout::new::<u8>());
        println!("after oom");
    });
    println!("{:?}", result);
}
