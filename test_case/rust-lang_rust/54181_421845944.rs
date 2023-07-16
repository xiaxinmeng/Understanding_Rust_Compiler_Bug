
	        e.span_suggestion_short_with_applicability(
                    self.span,
                    "use `&&` instead of `and` for the boolean operator",
                    "&&".to_string(),
                    Applicability::MaybeIncorrect,
                );
