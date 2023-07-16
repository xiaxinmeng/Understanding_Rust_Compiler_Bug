rust
const fn foo<T>() -> usize {
    0
}

fn ice<T>() {
    let _: [u8; foo::<*const T>()];
}
