
	impl Span {
		fn set_location_from(&mut self, location: Span);
		fn set_hygiene_context_from(&mut self, hygiene_context: Span);
	}
	