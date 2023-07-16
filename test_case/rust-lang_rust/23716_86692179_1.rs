 rust
static text: &str = "rust";

fn foo( text: i32 ) {
     print!("{}", text); // usage of the function parameter
     print!("{}", ::text); // usage of the static variable
}
