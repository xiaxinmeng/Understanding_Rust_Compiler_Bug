rust
// Since we expect for the mix of attributes used here to compile
// successfully, and we are just testing for the expected warnings of
// various (mis)uses of attributes, we use the `rustc_error` attribute
// on the `fn main()`.

#[rustc_error]
fn main() { //~ ERROR compilation successful
    println!("Hello World");
}
