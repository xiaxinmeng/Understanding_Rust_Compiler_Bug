
trait Foo {
    fn f(&self) -> int;
}

trait Bar : Foo {
    fn g(&self) -> int;
}

struct A {
    x: int
}

impl Foo for A {
    fn f(&self) -> int { 10 }
}

impl Bar for A {
    fn g(&self) -> int { 20 }
}

fn to_foo(x:&Bar) -> &Foo {
    unsafe { std::mem::transmute(x) }
}

pub fn main() {
    let a = &A { x: 3 };
    let abar = a as &Bar;
    let abarasafoo = to_foo(abar);

    assert_eq!(abarasafoo.f(), 20); // g is called 
    assert_eq!(abarasafoo.f(), 10); // g is called
