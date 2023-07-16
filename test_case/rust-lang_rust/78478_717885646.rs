rust
fn generic<T: Bok<N>, const N: usize>(v: T) {
    fn_taking_taking_dyn_foo::<N>(&v);
}

fn concrete<T: ?Sized + Foo<3>>(_: &T) {}
fn call_super_concrete(x: &dyn Bar<3>) {
    concrete(x);
}
