rust
impl<V, E, F> Result2 for Result<Result<V, F>, E> {
	type V = V;
	type E = E;
	type F = F;
	
	fn and_then2<V2, O>(self, op: O) -> Result<Result<V2, Self::F>, Self::E>
	where
		O: FnOnce(Self::V) -> Result<Result<V2, Self::F>, Self::E>,
	{
		match self {
			Ok(Ok(val)) => op(val),
			Ok(Err(fail)) => Ok(Err(fail)),
			Err(err) => Err(err),
		}
	}
	
	fn flatten(self) -> Result<Self::V, Self::E>
	where
		Self::F: Into<Self::E>,
	{
		self.flatten_with(|e| e.into())
	}
	
	fn flatten_with<O: FnOnce(Self::F) -> Self::E>(self, op: O) -> Result<Self::V, Self::E> {
		match self {
			Ok(Ok(v)) => Ok(v),
			Ok(Err(f)) => Err(op(f)),
			Err(e) => Err(e),
		}
	}
	
	fn map2<V2, O: FnOnce(Self::V) -> V2>(self, op: O) -> Result<Result<V2, F>, Self::E> {
		match self {
			Ok(r) => Ok(r.map(op)),
			Err(e) => Err(e),
		}
	}
	fn map_err2<F2, O: FnOnce(Self::F) -> F2>(self, op: O) -> Result<Result<Self::V, F2>, Self::E> {
		match self {
			Ok(r) => Ok(r.map_err(op)),
			Err(e) => Err(e),
		}
	}
	
	fn transpose(self) -> Result<Result<Self::V, Self::E>, Self::F> {
		match self {
			Ok(Ok(val)) => Ok(Ok(val)),
			Ok(Err(fail)) => Err(fail),
			Err(err) => Ok(Err(err)),
		}
	}
}
