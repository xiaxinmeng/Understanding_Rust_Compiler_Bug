rs
fn take(id: impl for<T> Fn(T) -> T) {
    id(0);
    id("");
}
