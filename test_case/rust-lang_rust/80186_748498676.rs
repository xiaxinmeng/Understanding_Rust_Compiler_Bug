rust
    /// Parse a mutable binding with the `mut` token already eaten.
    fn parse_pat_ident_mut(&mut self) -> PResult<'a, PatKind> {
        let mut_span = self.prev_token.span;

        if self.eat_keyword(kw::Ref) {
            return self.recover_mut_ref_ident(mut_span);
        }

        self.recover_additional_muts();

        // Make sure we don't allow e.g. `let mut $p;` where `$p:pat`.
        if let token::Interpolated(ref nt) = self.token.kind {
            if let token::NtPat(_) = **nt {
                self.expected_ident_found().emit();
            }
        }

        // Parse the pattern we hope to be an identifier.
        let mut pat = self.parse_pat(Some("identifier"))?;

        // If we don't have `mut $ident (@ pat)?`, error.
        if let PatKind::Ident(BindingMode::ByValue(m @ Mutability::Not), ..) = &mut pat.kind {
            // Don't recurse into the subpattern.
            // `mut` on the outer binding doesn't affect the inner bindings.
            *m = Mutability::Mut;
        } else {
            // Add `mut` to any binding in the parsed pattern.
            let changed_any_binding = Self::make_all_value_bindings_mutable(&mut pat);
            self.ban_mut_general_pat(mut_span, &pat, changed_any_binding);
        }

        Ok(pat.into_inner().kind)
    }
