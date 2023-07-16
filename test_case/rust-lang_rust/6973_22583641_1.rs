 rust
let foo = Some('a');
foo.unwrap();  // same as today
foo.unwrap('z');  // imagine that this replaced unwrap_or_default

int::from_str("4");  // same as today
int::from_str("4", 16);  // imagine that this replaced from_str_radix
