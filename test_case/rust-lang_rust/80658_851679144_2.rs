rust
        if let Some(exp_found) = exp_found {
            self.suggest_as_ref_where_appropriate(span, &exp_found, diag);
            self.suggest_accessing_field_where_appropriate(cause, &exp_found, diag);
            self.suggest_await_on_expect_found(cause, span, &exp_found, diag);
        }
