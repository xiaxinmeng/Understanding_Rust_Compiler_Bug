 rust
trait Foo { type Out; }
#[derive(Clone)]
struct S<T: Foo>(T::Out);
impl<T: Foo> Copy for S where T::Out: Copy {}

impl<'a> Foo for (&'a u32, &'a u32) {
    type Out = u32;
}
