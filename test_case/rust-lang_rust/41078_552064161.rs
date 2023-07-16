rust
fn filter<'a, P>(predicate: P)
where
    P: Fn(&'a S) -> bool,
{
    predicate(&S {});
}
