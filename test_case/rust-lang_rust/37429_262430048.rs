rust
pub const SIZEOF_QUERY:      usize = 21;

#[repr(C,packed)]
pub struct p0f_api_query {
	pub magic: u32,
	pub addr_type: u8,
	pub addr: [u8; 16],
}
