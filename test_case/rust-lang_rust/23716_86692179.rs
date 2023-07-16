 rust
static text : &str = "rust";

fn main() {
    match "rust compiler" {
        text => ... , // rather to use const text: &str = "rust"; 
        ....
    }
}
