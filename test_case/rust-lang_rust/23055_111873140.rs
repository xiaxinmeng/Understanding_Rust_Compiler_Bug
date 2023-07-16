 rust
fn remove(x: &mut Vec<u8>, index: usize) -> u8 {
    x.drain_range(index).next().unwrap()
}
