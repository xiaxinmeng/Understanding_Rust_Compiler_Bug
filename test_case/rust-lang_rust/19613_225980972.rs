 rust
struct A;
trait As {
    fn banana();
}

impl As for A {
    fn banana() { panic!("banana"); }
}

trait Bs<T: As> {
    fn do_it(&self) { T::banana() }
}

struct B;

impl Bs<A> for B {}

fn main() {
    Bs::do_it(&B)
}
