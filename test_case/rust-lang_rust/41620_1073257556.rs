rs
	match x / y
	{
		v if !v.is_finite() => None,
		v => Some(v)
	}
