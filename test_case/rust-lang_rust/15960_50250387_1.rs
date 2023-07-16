
struct Foo {
    bar: int
}
impl Rand for Foo {
    fn rand<R: Rng>(_: &mut R) -> Foo {
        Foo {
            bar: 4 //chosen by fair dice roll.
                   //guaranteed to be random.
        }
    }
}
impl Rem<int, int> for Foo {
    fn rem(&self, rhs: &int) -> int {
        self.bar % *rhs
    }
}
