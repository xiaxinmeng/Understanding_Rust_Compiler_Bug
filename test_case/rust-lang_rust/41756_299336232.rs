rust
pub trait Foo<T> { fn foo(&self) -> T; }
impl<T> Foo<()> for T { fn foo(&self) { } }
impl Foo<bool> for bool { fn foo(&self) -> bool { *self } }

pub fn foo<T>(t: T) where T: Foo<bool> {
   println!("{:?}", <T as Foo<_>>::foo(&t));
}
fn main() { foo(false); }
