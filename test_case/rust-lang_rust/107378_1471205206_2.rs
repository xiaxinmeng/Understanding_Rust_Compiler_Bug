rust
#![feature(anonymous_lifetime_in_impl_trait)]

trait Trait<'a> {
    fn foo(&self) -> &'a str { "" }
}

// Comment this to see the other two errors
pub fn f(t: impl Trait<'_>) -> &str {
    t.foo()
}

pub fn g(t: &impl Trait<'_>) -> &str {
    t.foo()
}

pub fn parse_reg(it: &mut impl Iterator<Item=&Token>) -> Result<&str, String> {
    let c = it.next().unwrap();
    match c {
        Token::Text(text) => Ok(text.as_str()),
        _ => Err(format!("Expected register got {:?}", c))
    }
}
