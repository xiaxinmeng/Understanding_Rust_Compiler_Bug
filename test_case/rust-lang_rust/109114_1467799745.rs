rust
fn main() {
    let mut chars = "£".chars();
    let c = chars.next().unwrap();

    if c.is_whitespace() {
        panic!("{:?} is whitespace", c);
    }
}
