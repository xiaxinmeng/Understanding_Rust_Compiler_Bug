 rust
fn main() {
    let mut xs = [1i, 2i, 3i];
    let x = 1 >> 128;

    xs[x] = 100i;
}
