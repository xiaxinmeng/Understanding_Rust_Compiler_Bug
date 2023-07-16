rust
const fn unwrap<T>(opt: Option<T>) -> T {
    match opt {
        Some(x) => x,
        None => panic!(),
    }
}
