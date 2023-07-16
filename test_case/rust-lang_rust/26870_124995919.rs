
// Almost PartialEq
trait PartialQe<Rhs = Self> {
    fn qe(&self, other: &Rhs) {}
}

impl<A, B> PartialQe<Option<B>> for Option<A> {}

fn main() {
    // `None` is not inferred to be `None::<&str>` based on the default `Rhs = Self`
    PartialQe::qe(&Some("str"), &None); // error: unable to infer enough type information about `_`; type annotations or generic parameter binding required [E0282]
}
