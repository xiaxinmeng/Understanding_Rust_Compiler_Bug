rust
fn main() {
    std::thread::spawn(|| panic!()).join().unwrap();
}
