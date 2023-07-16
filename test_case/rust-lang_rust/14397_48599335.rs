 rust
#![allow(dead_code)]

extern crate debug;

fn main() {
    struct Foo<type T> { y: T }

    let f : Foo<[int, ..3]> = Foo { y: [5i, 6, 7] };
    println!("&f.y: {:?}", &f.y);

    let g : Box<Foo<[int]>> = box f;
    println!("&g.y: {:?}", &g.y);
}
