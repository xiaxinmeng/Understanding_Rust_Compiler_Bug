rust
use std::mem;

#[repr(packed)]
#[derive(Copy, Clone, PartialEq, Debug)]
struct Foo {
    bar: u8,
    baz: u64
}

pub fn main() {
    let foos = [Foo { bar: 1, baz: 2 }; 10];

    assert_eq!(mem::size_of::<[Foo; 10]>(), 90);

    for i in 0..10 {
        println!("{:?}, {:?}", foos[i], Foo { bar: 1, baz: 2});
    }

    for &foo in &foos {
        println!("{:?}, {:?}", foo, Foo { bar: 1, baz: 2 });
    }

    assert!(false);
}
