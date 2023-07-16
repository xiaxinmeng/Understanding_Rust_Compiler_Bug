 rust
        let resolved_t = match unresolved_type_action {
            UnresolvedTypeAction::Error => {
                structurally_resolved_type(fcx, sp, t)
            }
            UnresolvedTypeAction::Ignore => {
                // We can continue even when the type cannot be resolved
                // (i.e. it is an inference variable) because `Ty::builtin_deref`
                // and `try_overloaded_deref` both simply return `None`
                // in such a case without producing spurious errors.
                fcx.resolve_type_vars_if_possible(t)
            }
        };
