 rust
fn main() {
    let mut x = 0.1;
    for z in (1..100) {
        x = x + 0.01;
        println!("{}", x);
    }
}
