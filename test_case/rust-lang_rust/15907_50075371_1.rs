 rust
fn filter<'r, A>(predicate: <'a>|&'a A|: 'r -> bool) -> Filter<'r, A, Self>
