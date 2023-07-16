 rust
// Almost PartialEq
trait PartialQe<Rhs = Self> {
        fn qe(&self, other: &Rhs) {}
}

impl<A, B = A> PartialQe<Option<B>> for Option<A> {}

fn main() {
        PartialQe::qe(&Some("str"), &None); 
        Some('a').qe(&None);
}
