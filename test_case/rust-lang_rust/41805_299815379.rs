Rust
trait Item {
	fn to_str(&self) -> &str;
	fn count() -> i32;
	fn rand() -> Self;
	fn rand_str() -> String {
		let x = Self::rand(); //~ ERROR the trait bound `Self: std::marker::Sized` is not satisfied
		String::from(x.to_str())
	}
}
