Rust
#![allow(unused)]

trait Foo {
    const BAR: Self;
}

impl Foo for i32 {
    const BAR: Self = 4;
}
impl Foo for i64 {
    const BAR: Self = 3;
}

struct Bar<T: ?Sized>(usize, std::marker::PhantomData<T>);

impl<T: ?Sized> Foo for Bar<T> {
    const BAR: Self = Bar(4, std::marker::PhantomData);
}

impl<T: ?Sized> Bar<T> {
    fn foo() {
        let x = Self::BAR.0;
        let x: &'static usize = &Self::BAR.0;
        let x: [i32; Self::BAR.0] = [1, 2];
    }
}

fn main() {
    let x: [i32; i32::BAR as usize] = [1, 2, 3, 4];
    let x: [i32; i64::BAR as usize] = [1, 2, 3];
}
