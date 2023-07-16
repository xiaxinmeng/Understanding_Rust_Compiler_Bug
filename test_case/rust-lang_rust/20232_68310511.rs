 rust
#![feature(if_let)]

struct Foo<T> { x: T }

impl<T> Index<uint,T> for Foo<T> {
    fn index(&self, _: &uint) -> &T {
        &self.x
    }
}

struct Bar<T> { x: T }

impl<T> Deref<T> for Bar<T> {
    fn deref(&self) -> &T {
        &self.x
    }
}

struct Baz<T> { x: T }

impl<T> Baz<T> {
    fn foo(&self) -> &T {
        &self.x
    }
}

fn main() {
    let u = 1u8;
    let mut v = Foo { x: Bar { x: Baz { x: Some(&u) } } };
    let i = v[0].foo();
    if let Some(ref i) = *i {
        v.x.x.x = None;
        println!("i = {}", i);
    }
}
