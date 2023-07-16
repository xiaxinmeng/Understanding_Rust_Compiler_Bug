 rust
my_string = my_collection.into_iter().fold(my_string, |s, v| { let _ = write!(s, "{}", v); s }); // yuck!
