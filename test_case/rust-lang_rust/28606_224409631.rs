 Rust
let mut x = &String::new();;
match x {
    ref mut s => *s = get_it();
}
