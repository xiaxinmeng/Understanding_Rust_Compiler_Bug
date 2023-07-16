rust
pub struct Example<I: Iterator> {
    iterator: either::Either<I, std::iter::Empty<I::Item>>,
}
