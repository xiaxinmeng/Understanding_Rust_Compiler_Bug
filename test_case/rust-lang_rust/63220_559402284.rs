Rust
struct X;
fn main() {
    let x = X;
    let fnonce = || x;
    let x = X;
    let fnonce_move = move || x;
}
