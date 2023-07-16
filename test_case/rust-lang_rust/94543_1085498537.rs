rust
fn closure_unary<T>() -> impl Fn(T) -> T {
    |a| -a
}

fn closure_binary<T>() -> impl Fn(T) -> T {
    |a| a+a
}
