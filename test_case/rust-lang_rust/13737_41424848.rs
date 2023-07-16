 rust
fn foo<V: CloneableVector<u8>>(v: V) {
    let v = v.into_owned();
    // do stuff with v
}
