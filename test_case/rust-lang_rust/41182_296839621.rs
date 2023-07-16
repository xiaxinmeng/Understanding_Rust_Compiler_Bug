Rust
static SLICE: [usize; 1] = [0; 1];

fn ice(_: &str) -> impl Iterator<Item=&usize> {
    SLICE.iter()
}
