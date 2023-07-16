rust
// `bar: fn(&B)`, `a: Result<A, E>`, `A: Sized + Deref<Target=B>`
// This fails but `&*a?` or `&&a?` or `a?.deref()` work
bar(&a?);
