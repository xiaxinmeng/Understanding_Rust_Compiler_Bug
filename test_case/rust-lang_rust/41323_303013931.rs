rust
#![feature(associated_consts)]

trait Evil<'a, T: 'static> {
    const EVIL: fn(&'a T) -> &'static T;
}

impl<'a, T: 'static> Evil<'a, T> for () {
    const EVIL: fn(&'a T) -> &'a T = identity;
}

fn identity<T: 'static>(s: &T) -> &T { s }

fn evil() -> &'static Box<i32> {
    let b = Box::new(0);
    <()>::EVIL(&b)
}


fn main() {
    println!("{}", **evil())
}
