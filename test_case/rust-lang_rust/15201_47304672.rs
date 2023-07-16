 rust
fn main() {
    println!("{}", std::mem::size_of_val(&1));  // prints 8 (assumes `int`?)
    println!("{}", std::mem::size_of_val(&1.0));  // prints 8 (assumes `f64`?)
}
