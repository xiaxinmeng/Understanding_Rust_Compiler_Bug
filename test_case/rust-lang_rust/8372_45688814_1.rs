 rust
fn main() {
    let s = "foobar bazbang";
    let mut chars = s.chars();

    for c in chars {
        if c == ' ' {
            chars.next();
        } else {
            print!("{}", c);
        }
    }
}
