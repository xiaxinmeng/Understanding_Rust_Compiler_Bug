 rust
        let a = try!(eval_const_expr_partial(tcx, &**a, ty_hint, fn_args));
       match (a, b, op.node) {
           // short circuit bool ops
           (Bool(false), _, BiAnd) => return Bool(false),
           (Bool(true), _, BiOr) => return Bool(true),
           _ => { }
       }
       // FIXME: lazy eval b?
       let b = eval_const_expr_partial(tcx, &**b, b_ty, fn_args);
       match (a, b, op.node) {
           (Bool(true), Ok(Bool(rhs)), BiAnd) => Bool(rhs),
           (Bool(false), Ok(Bool(rhs)), BiOr) => Bool(rhs),
            ... // as before
        }
