rust
fn test<T: Into<i32>>(t: T) -> i32 {
    5_i32 + t.into()
}
