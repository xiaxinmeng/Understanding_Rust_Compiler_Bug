rust
struct Foo<T: ?Sized>(T);

impl<T> Foo<T> {
    const BAR: () = ();
}

fn main() {
    let _: () = Foo::<[u8]>::BAR;
}
