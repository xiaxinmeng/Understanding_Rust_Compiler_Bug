rust
struct Foo<const N: usize>;

trait Bar<const N: usize> {}

impl<const N: usize> Foo<N> {
    fn unsatisfied(self, _: impl Bar<N>) {}
}

// Before nightly-2022-12-23.
fn trigger_old() {
    Foo.unsatisfied(());
}

// After nightly-2022-12-23 (inclusive).
fn trigger_new<const N: usize>() {
    Foo::<N>.unsatisfied(());
}
