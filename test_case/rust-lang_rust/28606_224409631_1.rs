 Rust
let mut x = &String::new();
match x {
    "hello" => {}
    ref mut s => *s = get_it();
}
