`rust
fn main() {
    let x = -2;
    let _a =  String::new().push_str(&String::from(" ").repeat(x as usize));
}
