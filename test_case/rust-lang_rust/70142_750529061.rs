rust
impl<V, E, F> Result<Result<V, F>, E> {
	fn flatten(self) -> Result<V, E>
		where
			F: Into<E>,
	{
		self.flatten_with(|e| e.into())
	}
		
	fn flatten_with<O: FnOnce(F) -> E>(self, op: O) -> Result<V, E> {
		match self {
			Ok(Ok(v)) => Ok(v),
			Ok(Err(f)) => Err(op(f)),
			Err(e) => Err(e),
		}
	}
}
