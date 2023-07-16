rust
fn main() {
    let s = String::from("Hello, world!");
    s.chars().filter(|c| !(" \n\t".contains(c))).collect::<String>();
}
