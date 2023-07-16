rust
struct Foo<const N: usize>;
impl Foo<3> {
    const SUM: () = ();
}

fn main() {
    Foo<3>::SUM
}
