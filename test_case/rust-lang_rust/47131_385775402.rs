rust
#![deny(warnings)]

type FooBar<F> = <F as Foo>::Bar;

trait Foo {
    type Bar;
}

struct Blop<F>(F);

impl<F: Foo> Blop<F>
where
    FooBar<F>: Clone,
{
}

impl Foo for () {
    type Bar = ();
}

fn main() {
    let _ = Blop(());
}
