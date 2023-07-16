 Rust
#![feature(specialization)]

trait Coerce<U> {
    fn coerce(self) -> Option<U>;
}

impl<U,V> Coerce<U> for V {
    default fn coerce(self) -> Option<U> { None }
}

impl<U> Coerce<U> for U {
    fn coerce(self) -> Option<U> { Some(self) }
}

fn coerce<V, U>(v: V) -> Option<U> {
    <V as Coerce<U>>::coerce(v)
}

fn coerce_lifetime<'a, 'b, T>(t: &'a T) -> &'b T {
    coerce(t).unwrap()
}

fn main() {
    let a = String::from("hello");

    let bomb = coerce_lifetime(&a);
    drop(a);
    println!("{}", bomb);
}

