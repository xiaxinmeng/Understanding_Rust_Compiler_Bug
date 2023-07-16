 rust
macro_rules! m { () => { let x = 0 } }
fn main() {
    m!(); // Is this OK? I think not, but it might be too common in the wild
          // to include in the warning cycle -- we haven't cratered it yet.
}
