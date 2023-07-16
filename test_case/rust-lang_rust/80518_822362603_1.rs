rust
pub fn munge<'a, 'b, Data>(self: &'b Xorcism<'a>, data: Data) -> impl Iterator<Item=u8> + 'b;
