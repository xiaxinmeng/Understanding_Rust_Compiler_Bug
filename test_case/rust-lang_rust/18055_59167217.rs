 rust
fn f(v: &[u8]) -> Box<Clone + 'static> {
    box v
}

fn main() { }
