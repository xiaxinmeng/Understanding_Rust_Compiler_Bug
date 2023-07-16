 rust
fn main() {
    for _ in 0 .. 2 {
        let x = break;
        x(); //~ERROR
    }
}
