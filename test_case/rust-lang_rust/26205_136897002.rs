 rust
// Returns the type and mutability of *ty.
//
// The parameter `explicit` indicates if this is an *explicit* dereference.
// Some types---notably unsafe ptrs---can only be dereferenced explicitly.
pub fn builtin_deref(&self, explicit: bool) -> Option<TypeAndMut<'tcx>> {
    match self.sty {
        TyBox(ty) => {
            Some(TypeAndMut {
                ty: ty,
                mutbl: ast::MutImmutable,
            })
        },
        TyRef(_, mt) => Some(mt),
        TyRawPtr(mt) if explicit => Some(mt),
        _ => None
    }
}
