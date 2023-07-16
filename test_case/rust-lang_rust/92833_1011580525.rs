
trait Trait<'a1, 'a2> {}
trait WithLifetimeAndAssoc<'x> {
    type Assoc;
}

fn foo<'a>(request: &'a str) -> impl for<'b> WithLifetimeAndAssoc<'b, Assoc = impl Trait<'a, 'b>> {
    todo!()
}
