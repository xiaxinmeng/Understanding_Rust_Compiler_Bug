 rust
fn main() {
    // each statement produces the same ICE
    let _: &mut String = &mut Vec::new();
    let _: &String = &Vec::new();
    let _: &() = &Vec::new();
}
