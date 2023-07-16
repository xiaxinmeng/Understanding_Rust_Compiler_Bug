rust
fn main() {
    std::thread::scope(|s| {
        s.spawn(|| panic!("!!"));
        panic!("!!!");
    });
}
