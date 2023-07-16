 rust
struct A<X>(X);

trait Foo: Sized {
    type Item;
    fn go(self) -> A<Self> {
        A(self)
    }
}

impl Foo for () {
    type Item = ();
}

#[cfg(equality)]
impl<X: Foo<Item=()>> Foo for A<X> {
    type Item = ();
}

#[cfg(not(equality))]
impl<X: Foo> Foo for A<X> {
    type Item = ();
}

fn main() {
    ().go().go().go().go()
      .go().go().go().go()
      .go().go().go().go()
      .go().go().go().go();
}
