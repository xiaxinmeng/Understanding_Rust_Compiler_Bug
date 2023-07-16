rust
fn caller<A>(x: A) -> usize {
    callee::<A>()
}

fn callee<B>() -> usize {
    size_of::<B>()
}
