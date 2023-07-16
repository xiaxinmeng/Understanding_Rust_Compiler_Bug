
trait Foo {
    type A;
}

struct Bar;

fn foo<I: Foo<A=Bar>>(x: &I) {}

fn foo<I>(x: &I)
    where I: Foo,
          I::A = Bar
{
}
