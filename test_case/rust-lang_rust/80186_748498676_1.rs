rust
    /// Turn all by-value immutable bindings in a pattern into mutable bindings.
    /// Returns `true` if any change was made.
    fn make_all_value_bindings_mutable(pat: &mut P<Pat>) -> bool {
        struct AddMut(bool);
        impl MutVisitor for AddMut {
            fn visit_pat(&mut self, pat: &mut P<Pat>) {
                if let PatKind::Ident(BindingMode::ByValue(m @ Mutability::Not), ..) = &mut pat.kind
                {
                    self.0 = true;
                    *m = Mutability::Mut;
                }
                noop_visit_pat(pat, self);
            }
        }

        let mut add_mut = AddMut(false);
        add_mut.visit_pat(pat);
        add_mut.0
    }
