
// Crate B
#[repr(transparent)]
pub struct NeedsToken(u32, crate_a::Token);
