rust
fn main() {
    for f in -5..5 {
        println!("{0} -> {0:.0}", f as f64 + 0.5f64);
    }
}
