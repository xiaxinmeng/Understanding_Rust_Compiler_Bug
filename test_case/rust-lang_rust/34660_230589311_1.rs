 rust
macro_rules! n { () => {
    let _ = ();
    m!() // this is no longer considered to be in a statement position like it was
         // before this PR, which could potentially lead to more breakage.
}
fn main() { n!(); }
