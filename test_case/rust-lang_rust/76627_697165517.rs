rust
fn main() {
    let c = |v| {};
    for _ in 0..1 {
        let s = ();
        c(&s);
    }
}
