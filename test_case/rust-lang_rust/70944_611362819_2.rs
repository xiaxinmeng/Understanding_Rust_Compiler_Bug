
fn do_stuff(foo: &impl FooBuilder) {
    let inner = foo.inner();
    Index::<KeyA>::index(inner, KeyA);
    Index::<KeyB>::index(inner, KeyB);
    Index::<KeyC>::index(inner, KeyC);
    // works just fine
}
