 rust
fn main() {
    let child = std::thread::spawn(|| {
        std::cell::RefCell::new([0i8; 232500]);
    });
    child.join().unwrap();
}
