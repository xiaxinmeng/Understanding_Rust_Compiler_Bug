rust
const fn takes_string(value: String) -> usize {
     value.len() + 420 / 10
} // the size of value is already known, therfore it can be const?

let mut string = String::new();
string.push_str("Hello");

takes_string(string);
