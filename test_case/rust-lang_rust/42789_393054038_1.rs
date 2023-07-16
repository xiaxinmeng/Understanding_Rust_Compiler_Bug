rust
use unique_token::Token;

fn main() {
    let t = Token::new().expect("Couldn't even get a single token");
    // make sure we can't get a second
    assert!(Token::new().is_none());
    
    let s = &[t];
    let t1 = &s[0];
    let t2 = s.iter().next().expect("Slice can't be empty");
    Token::take_two(t1, t2);
}
