
fn do_stuff(foo: &impl FooBuilder) {
    let inner = foo.inner();
    Index::index(inner, KeyA);
    //                  ^^^^ expected struct `KeyC`, found struct `KeyA`
    Index::index(inner, KeyB);
    //                  ^^^^ expected struct `KeyC`, found struct `KeyB`
    Index::index(inner, KeyC);
}
