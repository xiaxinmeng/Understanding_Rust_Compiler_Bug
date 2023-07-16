
	impl Span {
		fn with_location_from(&self, location: Span) -> Span;
		fn with_hygiene_context_from(&mut self, hygiene_context: Span) -> Span;
	}
	