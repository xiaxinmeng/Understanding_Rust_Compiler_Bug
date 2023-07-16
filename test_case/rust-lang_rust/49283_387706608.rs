rust
fn main() {
    let s = "e\u{301}";
    println!("str: {:?}", s); // str: "é"
    println!("bytes: {:?}", s.chars().collect::<Vec<_>>()); // bytes: ['e', '́']
}
