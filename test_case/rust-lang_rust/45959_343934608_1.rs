rust
self.fully_perform_op(location.at_self(), |this| {
    let mut selcx = traits::SelectionContext::new(this.infcx);
    let cause = traits::ObligationCause::misc(this.last_span, ast::CRATE_NODE_ID);
    let obligation = traits::Obligation::new(cause, this.param_env, trait_ref.to_poly_trait_ref());
    if let Some(vtable) = selcx.select(&obligation)? {
        Ok(InferOk { value: (), obligations: vtable.nested_obligations() })
    } else {
        mir_span_bug!(...) // typeck should have ensured that this will succeed
    }
}).unwrap()
