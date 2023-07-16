rust
pub struct Foo {
	pub foo: i32,
}

pub mod bar {
	use crate::other::Foo;

	impl crate::Foo {
		/// Baz the [`Self::foo`].
		pub fn baz(&self) {
			println!("bazzing the foo");
		}
	}
}

pub mod other {
	pub struct Foo;
}
