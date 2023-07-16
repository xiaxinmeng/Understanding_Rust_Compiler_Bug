rust
fn main() {
    let v: Vec<_> = std::iter::empty().chain([42]).collect();
    println!("{:?}", v); // prints [42]
}
