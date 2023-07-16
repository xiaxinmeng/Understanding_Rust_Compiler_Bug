 rust
impl<Lhs, Rhs> PartialEq<Rhs> for Lhs where Rhs: PartialEq<Lhs> {
    fn eq(&self, rhs: &Rhs) -> bool {
        rhs.eq(self)
    }
}

struct Foo;
struct Bar;

fn overflow() {
    Foo == Bar;
}
