rust
pub fn foo<T: 'static>() -> Box<Iterator<Item = [T; 17]>> {
    use std::iter::empty;

    Box::new(empty()
        .chain(empty())
        .chain(empty())
        .chain(empty())
        .chain(empty())
        .chain(empty())
        .chain(empty())
        .chain(empty())
        .chain(empty())
        .chain(empty())
        .chain(empty()) // 10th .chain(empty())
        .chain(empty())
        .chain(empty())
        .chain(empty())
        .chain(empty())
        .chain(empty())
        .chain(empty())
        .chain(empty()) // 17th .chain(empty())
    )
}
