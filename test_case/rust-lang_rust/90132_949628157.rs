rust
fn main() {
    if let arg = std::env::args().nth(1).next() {
        println!("{}", arg);
    } else {
        println!("no argument");
    }
}
