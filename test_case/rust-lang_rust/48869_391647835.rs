rust
struct A;

impl std::ops::Add for Box<A> {
    type Output = Box<A>;
    fn add(self, rhs: Box<A>) -> Self::Output {
        Box::new(A)
    }
}
