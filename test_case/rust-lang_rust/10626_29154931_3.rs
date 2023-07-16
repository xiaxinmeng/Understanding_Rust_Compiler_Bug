
// child.rs
fn main() {
    for _ in range::<uint>(0, 1000) {
        writeln!(&mut std::io::stderr() as &mut std::io::Writer, "hello?");
    }
}
