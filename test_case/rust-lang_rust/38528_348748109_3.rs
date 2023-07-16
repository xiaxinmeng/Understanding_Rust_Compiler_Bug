rust
pub fn foo() -> Box<Iterator<Item = ()>> {
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
