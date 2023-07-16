 rust
macro_rules! m { () => { 2 } } //< The only difference is that the `()` is changed to a `2`
macro_rules! n {
    () => { m!() - 1 }
}
fn main() { n!(); }
