Rust
static SLICE: [usize; 1] = [0; 1];

fn clear_error_message() -> impl Iterator<Item=&usize> {
    SLICE.iter()
}
