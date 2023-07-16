rust
use std::borrow::Borrow;
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Foo<'a> {
    val: i32,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Foo<'a> {
    fn new(i: &'a i32) -> Self {
        Foo {val: *i, phantom: PhantomData}
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Key(Foo<'static>);

impl Key {
    fn new(x: Foo) -> Key {
        Key(Foo {val: x.val, phantom: PhantomData})
    }
}

impl<'a> Borrow<Foo<'a>> for Key {
    fn borrow(&self) -> &Foo<'a> {
        &self.0
    }
}

fn main() {
    let mut map: HashMap<Key, i32> = HashMap::new();
    map.insert(Key::new(Foo::new(&4)), 5);
    assert!(map.get(&Foo::new(&4)).is_some());
    println!("{:?}", map);
    assert!(map.remove(&Foo::new(&4)).is_some());
    println!("{:?}", map);
}
