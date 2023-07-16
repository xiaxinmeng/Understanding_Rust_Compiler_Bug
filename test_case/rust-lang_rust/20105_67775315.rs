 rust
fn main() {
    std::task::spawn(move || -> ! { panic!("A") });
}
