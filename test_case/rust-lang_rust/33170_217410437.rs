
fn foobar<I, T: Into<Option<T>>>(t: T) -> Option<T> {
    t.into()
}
