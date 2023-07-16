 rust
fn a() { }
fn b() { }

fn main() {
    let c = if true { a } else { b };
    c();
}
