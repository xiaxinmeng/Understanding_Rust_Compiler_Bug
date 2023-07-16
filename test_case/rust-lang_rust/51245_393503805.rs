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
        let v = Vec::<u8>::with_capacity(100000000000000);
        println!("{:p}", &v[..]);
    });
    println!("{:?}", result);
}
