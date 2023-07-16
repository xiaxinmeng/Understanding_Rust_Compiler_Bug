rs
use weird_lib::*;

struct Wrapped<'a>(Foo<'a>);
impl Clone for Wrapped<'_> {
    fn clone(&self) -> Self {
        panic!("please don't reach this code!")
    }
}
// properly only on `'static`
impl Copy for Wrapped<'static> {}

fn main() {
    let s = Foo::with_new(|ref mut foo| {
        let mut x = vec![Wrapped(foo.take().unwrap())];
        let mut y = x.clone(); // <- doesn't actually call `clone` on `Wrapped` due to specialization
        Foo::proved_static(&mut x[0].0, &mut y[0].0)
    });
    println!("{}", s.into_inner());
}
