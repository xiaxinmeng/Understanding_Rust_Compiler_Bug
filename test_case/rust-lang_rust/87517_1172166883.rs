rust
#[derive(Default)]
pub enum Either<A, B> {
    #[default]
    One(A),
    Two(B),
}
