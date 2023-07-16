 rust
// make some string that isn't necessarily exactly what you want
let s: String = foo();
// convert it into the form you need, borrowing out of the original
let s: &str = s.trim();
