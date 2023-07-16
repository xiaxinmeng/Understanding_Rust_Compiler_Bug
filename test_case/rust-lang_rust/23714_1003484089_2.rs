rust
enum Array<'a, T> {
    Borrowed(&'a [T]),
    Owned(Vec<T>),
}

struct Element {
    children: Array<'static, Element>,
}
