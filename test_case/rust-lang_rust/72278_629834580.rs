rust
struct S;

impl S {
    fn func<'a, U>(&'a self) -> U {
        todo!()
    }
}

fn crash<'a, U>() -> U {
    S.func::<'a, U>()
}
