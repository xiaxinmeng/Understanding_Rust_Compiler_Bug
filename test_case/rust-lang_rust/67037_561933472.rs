rust
struct U {}
struct P<T>(T);

fn main() {
    let P() = U {}; //~ ERROR mismatched types
}
