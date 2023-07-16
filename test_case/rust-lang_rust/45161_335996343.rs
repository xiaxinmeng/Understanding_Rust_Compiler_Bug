rust
enum Enum { A }

fn fail() {
    let x: Vec<_> = [Enum::A].iter().map(|e| e).collect();
}
