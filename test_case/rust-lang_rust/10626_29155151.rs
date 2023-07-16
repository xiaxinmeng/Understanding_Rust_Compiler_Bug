
// child.rs
// exits normally
fn main() {
    let mut w = std::io::stderr();
    for _ in range::<uint>(0, 1000) {
        w.write("hello?\n".as_bytes());
    }
}
