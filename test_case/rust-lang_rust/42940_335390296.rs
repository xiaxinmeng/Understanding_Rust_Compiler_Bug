rust
#![feature(conservative_impl_trait)]

trait Tr { }

struct S;
impl Tr for S { }

fn foo<T>(_t: T) -> impl Tr {
    S
}

struct S2;

fn main() {
    let _bar = {
        let s2 = S2;
        foo(&s2)
    };
}
