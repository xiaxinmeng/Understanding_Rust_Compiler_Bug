rust
fn main() {
    let f = File::create(...);
    let f2 = File::from_raw_fd(f.as_raw_fd());
    let m = MemoryMap::new(f).unwrap();
    drop(f2);
    // Use m and segfault...
}
