Rust
pub fn parse_headers<'b: 'h, 'h>(src: &'b [u8], mut dst: &'h mut [Header<'b>])
