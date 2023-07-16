 rust
fn foo<V: IntoVec<u8>>(v: V) {
    let v = v.into_vec();
    // do stuff with v
}
