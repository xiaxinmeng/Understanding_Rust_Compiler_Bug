 Rust
pub trait Foo for Sized? { fn foo<T>(&self, ext_thing: &T); }
pub trait Bar for Sized?: Foo { }
impl<T: Foo> Bar for T { }

pub struct Thing;
impl Foo for Thing {
    fn foo<T>(&self, _: &T) {}
}

#[inline(never)] fn foo(b: &Bar) { b.foo(&0u) }

fn main() {
    let mut thing = Thing;
    let test: &Bar = &mut thing;
    foo(test);
}
