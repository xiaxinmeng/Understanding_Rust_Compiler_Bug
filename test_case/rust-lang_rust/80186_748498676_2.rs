rust
    /// Error on `mut $pat` where `$pat` is not an ident.
    fn ban_mut_general_pat(&self, lo: Span, pat: &Pat, changed_any_binding: bool) {
        let span = lo.to(pat.span);
        let fix = pprust::pat_to_string(&pat);
        let (problem, suggestion) = if changed_any_binding {
            ("`mut` must be attached to each individual binding", "add `mut` to each binding")
        } else {
            ("`mut` must be followed by a named binding", "remove the `mut` prefix")
        };
        self.struct_span_err(span, problem)
            .span_suggestion(span, suggestion, fix, Applicability::MachineApplicable)
            .note("`mut` may be followed by `variable` and `variable @ pattern`")
            .emit();
    }
