
// child.rs
// crashes
fn main() {
    for _ in range::<uint>(0, 1000) {
        std::io::stderr().write("hello?\n".as_bytes());
    }
}
