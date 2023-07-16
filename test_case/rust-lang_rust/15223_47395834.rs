
fn foo<A : Shl<A, A>>(a: A) -> A {
    a << a
}

fn main() {
    foo(1u32);
}
