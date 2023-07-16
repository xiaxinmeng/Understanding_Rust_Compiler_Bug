Rust
impl<T> ConstPanic for Vec<T> {
    const EVAL: () = panic!("oh no!");
}
