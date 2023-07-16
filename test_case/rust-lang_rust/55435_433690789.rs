rust
fn main() {
    macro_rules! my_fn {
        ( $fn:tt ) => (
            println!("Use the {} function", $fn);
        );
    }
    my_fn!( "test" );
}
