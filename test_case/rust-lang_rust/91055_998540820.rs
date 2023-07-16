rust
fn foo<T: Into<i32>, U>(x: T) -> i32 {
    let x = |y: i32| y + x.into();
    x(3)
}
