rust
// Currently requires `where 'a: 'b, Self: 'a, Self: 'b`
// And none of those parameters "touch" the GAT
pub trait Gat<'a, 'b> {
    type Item<'c>;
    fn f(_: &'b &'a Self) -> Self::Item<'static>;
}
