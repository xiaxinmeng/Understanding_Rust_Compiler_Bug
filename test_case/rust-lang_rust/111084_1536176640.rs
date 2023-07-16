rust
const S: &'static str = {unreachable_unchecked(); 5};
pub const BYTES: &[u8] = &[0; { S.len() }];
