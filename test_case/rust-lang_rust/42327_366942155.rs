rust
impl std::ops::Try for Value {
	type Ok = Self;
	type Error = Self;
	
	fn from_ok(v: Self::Ok) -> Self { v }
	fn from_error(v: Self::Error) -> Self { v }
	fn into_result(self) -> Result<Self::Ok, Self::Error> {
		if self.is_ok() { Ok(val) } else { Err(val) }
	}
}
