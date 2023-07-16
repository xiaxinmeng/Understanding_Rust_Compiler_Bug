 rs
struct Foo<const A: usize, const B: usize = 0>;

impl<const B: usize> Foo<0, B> {
    fn mk() -> Self { Self }
}

fn main() {
    // complains about unspecified B
    let f = Foo::mk();

    // works, and that zero is not B's
    let f: Foo<0> = Foo::mk();
}
