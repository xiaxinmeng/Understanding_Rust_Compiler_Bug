 rust
left_ty = match left_ty {
    ast::PatIdent(ByRef, _, _) => ty::deref(left_ty, true).unwrap().ty,
    _ => left_ty
};
