
// Crate A
#[repr(transparent)]
pub struct Token(std::marker::PhantomData<()>);

impl Token {
	pub unsafe fn new() -> Self {
		Self(std::marker::PhantomData)
	}
}
