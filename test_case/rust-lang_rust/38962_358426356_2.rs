rust
// works now too despite [u8]::ToOwned being Vec<u8> not Box<[u8]>
let new_style: Cow<[u8], Box<[u8]>>;
