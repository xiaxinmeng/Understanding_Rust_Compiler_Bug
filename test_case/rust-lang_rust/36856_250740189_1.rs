 rust
fn main() {
    let a = !g();
    let b = !g();
    if a != b {
        println!("wrong");
    } else {
        println!("correct");
    }
}
