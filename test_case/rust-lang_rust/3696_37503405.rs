 rust
pub fn main() {
    let a = ~[~""];
    let b: ~[~[&str]] = a.map(|s| s.lines().collect()); // error: cannot infer an appropriate lifetime for autoref due to conflicting requirements
}
