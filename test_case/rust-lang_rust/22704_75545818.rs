 rust
mod foo {
    pub struct Bar<T1, T2> {
        t1: T1,
        t2: T2,
    }
}

struct Bar<T> {
    t: T,
}
