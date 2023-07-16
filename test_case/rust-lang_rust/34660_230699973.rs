 rust
macro_rules! m { () => { () } }
macro_rules! n {
    () => { m!() - 1 } // This is treated like "m!(); -1" today
}
fn main() { n!(); }
