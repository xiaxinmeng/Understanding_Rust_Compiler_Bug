rust
#![feature(const_fn)]
#![feature(associated_consts)]

#[derive(PartialEq, Eq)]
struct Thing {
    num: u8,
}

impl Thing {
    const fn new(num: u8) -> Thing {
        Thing { num: num }
    }
    
    const ZERO: Thing = Thing::new(0);
}

fn main() {
    let thing = Thing { num: 0 };
    
    match Some(thing) {
        Some(Thing::ZERO) => println!("Hi"),
        _ => println!("Hello")
    }
}
