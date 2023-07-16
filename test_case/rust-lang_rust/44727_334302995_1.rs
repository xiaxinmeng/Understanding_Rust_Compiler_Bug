rust
fn foo<'a,'b,T>(...) -> impl Iterator<Item = &'a T> { .. }
// for now imagine 'a and 'b are both early bound
