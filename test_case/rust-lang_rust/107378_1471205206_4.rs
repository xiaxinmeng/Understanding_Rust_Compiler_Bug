rust

error: lifetime may not live long enough
  --> src/lib.rs:15:5
   |
14 | pub fn g(t: &impl Trait<'_>) -> &str {
   |          -  - let's call the lifetime of this reference `'2`
   |          |
   |          has type `t`
15 |     t.foo()
   |     ^^^^^^^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`

error: lifetime may not live long enough
  --> src/lib.rs:27:30
   |
24 | pub fn parse_reg(it: &mut impl Iterator<Item=&Token>) -> Result<&str, String> {
   |                  --  - let's call the lifetime of this reference `'2`
   |                  |
   |                  has type `it`
...
27 |         Token::Text(text) => Ok(text.as_str()),
   |                              ^^^^^^^^^^^^^^^^^ function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`

error: could not compile `playground` (lib) due to 2 previous errors
