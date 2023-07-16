
// Crate A
#[repr(transparent)]
pub struct Token(());

impl Token {
	pub unsafe fn new() -> Self {
		Self(())
	}
}
