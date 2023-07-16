 rust
fn main() {
    let foo = ~"hello";
    let foo: ~[&str] = foo.words().collect();
    let invalid_string = foo[0];
    println!("{}", invalid_string);
}
