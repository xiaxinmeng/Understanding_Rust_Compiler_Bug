rust
enum Void {}

fn main() {
    let x: [Void; 0] = [];
    for _ in &x {}
}
