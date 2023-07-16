Rust
#![feature(existential_type)]

existential type Foo: Copy;

const BAZ: *const Foo = &0u8 as *const _ as *const _;

fn bar() -> Foo { 0u32 }

fn main() {
    let _ = BAZ;
}
