
fn somefunction(s: impl Into<String>) {
    println!("a String: {}", s.into())
}

match &someenum {
SomeCase { string_field } => somefunction(string_field), // string_field: &String
...
}
