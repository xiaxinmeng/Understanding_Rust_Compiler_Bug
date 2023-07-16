rust
fn foo<const N: usize>() -> [u8; N] {
    bar::<N>()
}

fn bar<const N: u8>() -> [u8; N] {}
