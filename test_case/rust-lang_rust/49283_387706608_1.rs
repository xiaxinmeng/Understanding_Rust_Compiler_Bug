rust
fn main() {
    let s = "e\u{301}";
    println!("str: {:?}", s); // str: "é" (unchanged)
    println!("bytes: {:?}", s.chars().collect::<Vec<_>>()); // bytes: ['e', '\u{301}']
}
