 rust
fn g<F:Foo>(f:&F) {
    f.foobar();
}

fn f<FB:FooBase>(fb:&FB) {
    g(fb);
}

fn main() {
    let x = X;
    f(&x);
}
struct X;
impl FooBase for X {}

trait Foo { fn foobar(&self); }

trait Bar { fn foobar(&self); }

trait FooBase {}
trait BarBase {}

impl<T: FooBase> Foo for T {
    fn foobar(&self) {}
}

impl<T: BarBase> Bar for T {
    fn foobar(&self) {}
}
