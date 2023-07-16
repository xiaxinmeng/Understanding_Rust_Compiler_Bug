rust
trait Foo { fn foo(self); }

struct S;
impl Foo for &S { fn foo(self) {} } // just to prove there is a way to bottom out

impl<'a, T> Foo for &'a Vec<T> where &'a T: Foo {
    fn foo(self) {}
}

fn array<'a, T>(a: &'a [T]) where &'a T: Foo {}

pub fn f() {
    let g = [1, 2, 3];
    array(&g);
}
