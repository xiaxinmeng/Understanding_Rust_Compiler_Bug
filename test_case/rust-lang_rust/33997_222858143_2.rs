 rust
extern crate doesnt_exist; //< We need to know if this crate has a top-level module `bar` ...
mod bar {
    macro_rules! m { () => {} }
}
fn main() {
    use doesnt_exist::*; //< ... (which would be imported here) ...
    bar::m!(); //< ... before we can expand this.
}
