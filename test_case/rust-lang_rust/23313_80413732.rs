 rust
let left_ty = match (&real_pat.node, &left_ty.sty) {
    (&ast::PatIdent(..), &ty::ty_rptr(_, ty::mt { ty, .. })) => match ty.sty {
        ty_int(..) | ty_uint(..) | ty_bool | ty_char | ty_float(..) => ty,
        _ => left_ty,
    },
    _ => left_ty,
};
