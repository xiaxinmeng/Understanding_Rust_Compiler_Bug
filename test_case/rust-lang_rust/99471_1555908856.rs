rust
fn f<const N: usize>() {
    if N == 0 {
        ...
    } else {
        foo.as_chunks::<N>()...
    }
}

fn main() {
    f::<0>();
}
