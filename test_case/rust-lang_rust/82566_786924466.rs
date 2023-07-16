rust
struct Foo<const N: usize, const M: usize>;
impl Foo<3, 4> {
    const SUM: () = ();
}

fn main() {
    Foo<3, 4>::SUM
}
