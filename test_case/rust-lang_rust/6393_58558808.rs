
use std::collections::SmallIntMap;

enum Foo<'a>{ A(&'a mut SmallIntMap<uint>), B(&'a mut uint) }

fn main() {
    let mut map = SmallIntMap::<uint>::new();
    do_stuff(&mut map);
}

fn do_stuff(map: &mut SmallIntMap<uint>) -> Foo {
    match map.find_mut(&1) {
        None => {},  // Definitely can't return A here because of lexical scopes
        Some(val) => return B(val),
    }
    return A(map); // ERROR: borrowed at find_mut???
}
