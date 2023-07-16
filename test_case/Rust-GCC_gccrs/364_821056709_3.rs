
NeverType::NeverType(Hirid ref, HirId ty_ref, ... HirId fallback_hir_id);

BaseType* get_fallback() const { return TyVar(fallback_hir_id).get_tyty(); )

private:
fallback_hir_id
