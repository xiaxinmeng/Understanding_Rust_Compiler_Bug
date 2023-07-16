
fn returns_result() -> Result<_,_>;

let result = try {
	returns_result()?;
	returns_result()
};
