rust
fn main() {
    let mut s = String::new();
    s += {&String::new()};
//        ^^^^^^^^^^^^^^ expected str, found struct `std::string::String`
// note: expected type `&str`
//          found type `&std::string::String`
}  
