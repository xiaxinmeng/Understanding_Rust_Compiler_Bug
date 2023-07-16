rust
let mut vec = vec![String::from("foo")];
vec.remove_item("foo");
//              ^^^^^ expected struct `std::string::String`, found `str`
