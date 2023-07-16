
fn do_stuff(foo: &impl FooBuilder) {
    let inner = foo.inner();
    inner.index(KeyA);
    //          ^^^^ expected struct `KeyC`, found struct `KeyA`
    inner.index(KeyB);
    //          ^^^^ expected struct `KeyC`, found struct `KeyB`
    inner.index(KeyC);
}
