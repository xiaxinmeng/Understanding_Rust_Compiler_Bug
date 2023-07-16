rust
trait Tr {
    type A;
}

impl<A> Tr for A {
    type A = A;
}

fn f() -> impl Tr<A=u8> {
    ()
}
