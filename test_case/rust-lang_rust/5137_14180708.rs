 rust
struct Defer {}
impl Drop for Defer {
    fn finalize(&self) { io::println("destroyed"); }
}
fn main() {
    let d = Defer{};
}
