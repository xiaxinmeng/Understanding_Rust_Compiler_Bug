rust
fn main() {
    let x: !;
    println!("asdf");
    *(&writeOnly x as &writeOnly ZST) = ZST {};
}
