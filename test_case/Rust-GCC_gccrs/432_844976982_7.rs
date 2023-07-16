rust=
struct Foo<A> (A);

impl Foo<isize> {
    fn test() -> ...
}

impl Foo<f32> {
    fn test() -> ...
}

fn main() {
    let a:i32 = Foo::test();
}
