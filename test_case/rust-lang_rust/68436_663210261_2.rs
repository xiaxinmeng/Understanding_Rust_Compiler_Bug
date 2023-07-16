rust
const fn foo(v: usize) -> usize {
    if v > 0 {
        25 / v
    } else {
        0
    }
}

fn bar<const N: usize>() where (foo::<N>()) {
    let _: [u8; foo::<N>()]
}
