 rust
fn main() {
    let s = "foobar bazbang";
    let mut chars = s.chars();

    loop {
        match chars.next() {
            Some(c) => {
                if c == ' ' {
                    chars.next();
                } else {
                    print!("{}", c);
                }
            },
            None => { break; }
        }
    }
}
