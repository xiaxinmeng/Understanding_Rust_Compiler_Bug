
trait MyTrait<T> { }

struct MyContainer<'self, T> {
    elems: ~[&'self MyTrait<T>],
}

impl<'self, T> MyContainer<'self, T> {
    fn foo (&self, f: &fn (&'self MyTrait<T>)) {
        f(self.elems[0]);
    }
    fn bar (&self) -> &'self MyTrait<T> {
        self.elems[0]
    }
}

fn main () {}
