
    fn report_mismatched_types(sp: span, e: ty::t, a: ty::t,
                               err: &ty::type_err) {
        // Don't report an error if expected is ty_err
        let resolved_expected =
            self.resolve_type_vars_if_possible(e);
        let mk_msg = match ty::get(resolved_expected).sty {
            ty::ty_err => return,
            _ => {
                // if I leave out : ~str, it infers &str and complains
                |actual: ~str| {
                    fmt!("mismatched types: expected `%s` but found `%s`",
                         self.ty_to_str(resolved_expected), actual)
                }
            }
        };
        self.type_error_message(sp, mk_msg, a, Some(err));
    }
