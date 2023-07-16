 rust
#![allow(dead_code)]

extern crate debug;

fn main() {
    struct Foo<type T> { y: T }

    let f : Foo<[int, ..3]> = Foo { y: [5i, 6, 7] };
    println!("&f.y: {:?}", &f.y);

    let g : &Foo<[int]> = &f;
    println!("&g.y: {:?}", &g.y);

    let h = &Foo { y: *g };
    println!("&h.y: {:?}", &h.y);
}
