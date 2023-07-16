rust
    match user_ty {
        UserTypeAnnotation::Ty(canonical_ty) => ...,
        UserTypeAnnotation::FnDef(def_id, canonical_substs) => ...,
        ...
    }
