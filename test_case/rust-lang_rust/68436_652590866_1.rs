rust
fn a<const N: usize>() {
    let _: [u8; N + 10];
}

fn foo<const N: usize>() {
    a::<{N * 2>()
}

fn main() {
    foo::<{usize::MAX / 2}>()
}
