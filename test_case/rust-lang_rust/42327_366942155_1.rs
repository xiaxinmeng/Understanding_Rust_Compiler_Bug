rust
impl std::ops::Try for StrandFail<T> {
	type Ok = T;
	type Error = Self;
	
	fn from_ok(v: Self::Ok) -> Self { StrandFail::Success(v) }
	fn from_error(v: Self::Error) -> Self { v }
	fn into_result(self) -> Result<Self::Ok, Self::Error> {
		match self {
			StrandFail::Success(v) => Ok(v),
			other => Err(other),
		}
	}
}
