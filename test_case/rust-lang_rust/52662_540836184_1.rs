rust
fn foo<T: FromStr>(a: T) where <T as FromStr>::Err: Debug {}
