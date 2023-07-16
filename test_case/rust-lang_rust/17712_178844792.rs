 rust
fn main() {
    let mut foo = Vec<i32>::new(); // Remove <32i> to fix
    foo.push(1);
    println!("{}", foo[0]);
}
