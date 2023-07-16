rust
type Curried<A> = impl std::ops::Deref<Target = impl FnOnce(A)>;

fn curry() -> Curried<()> {
    Box::new(drop)
}

fn main() {}
