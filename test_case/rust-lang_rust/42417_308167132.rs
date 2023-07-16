rust
// Given:
//     fn foo<A>() -> A
// Produce:
impl<R, A> FnOnce for Foo<A>
where A: Subtype<R>
{
    type Output = R;
}
