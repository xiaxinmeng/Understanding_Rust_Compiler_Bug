rust
trait Bar<T> { fn dummy(&self); }

trait Foo {
    type A;
    type B: Bar<Self::A>;

    fn get_b(&self) -> &Self::B;
}

fn test_bar<A, B: Bar<A>>(_: &B) {}

fn test<A, F: Foo<A=A>>(f: &F) {
    test_bar(f.get_b());
}

fn main() { }
