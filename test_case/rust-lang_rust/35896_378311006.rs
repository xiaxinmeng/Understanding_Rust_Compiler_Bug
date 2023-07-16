rust
// crate foo
extern crate bar;
use bar::baz; // works
use bar::bazmod::baz; // errors
fn main() { baz!() }

// crate bar
pub mod bazmod {
    #[macro_export]
    macro_rules! baz { () => { println!("hello world!"); } }
}
