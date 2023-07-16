rust
#![allow(unused)]
use std::mem;

#[derive(Copy, Clone)]
enum Never {}

union Foo {
    a: u64,
    b: Never
}

fn main() {
    println!("{}", mem::size_of::<Foo>());
    
    let f = [Foo { a: 42 }, Foo { a: 10 }];
    println!("{:?}", unsafe { f[0].a });
}
