rust
enum Foo<A = (), B = ()> {
    A(A),
    B(B),
}

fn main() {
    let a = Foo::A(());
}
