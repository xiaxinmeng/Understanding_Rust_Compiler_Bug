rust
#![feature(type_alias_impl_trait)]
trait Trait<'a> { type Out: 'a;}
impl<'a> Trait<'a> for i32 { type Out = &'a String;}
type A = impl for<'a> Trait<'a, Out = impl Sized + 'a>;
fn foo() -> A {
    0_i32
}
