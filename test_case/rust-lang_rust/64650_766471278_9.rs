rust
fn a(a: &str) -> &str {
    ""
}

fn main() {
    let a1 = a;
    let a2: fn(&str) -> &str = a;
}
