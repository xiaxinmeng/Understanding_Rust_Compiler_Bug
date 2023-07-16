rust
fn foo<T: FromStr>(a: T)
where T::Err: Debug {}
