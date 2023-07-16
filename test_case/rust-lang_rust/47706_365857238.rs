
enum Foo {
    Bar(i32),
}

fn foo<F>(f: F)
where
    F: Fn(),
{
}

fn main() {
    foo(Foo::Bar);
}
