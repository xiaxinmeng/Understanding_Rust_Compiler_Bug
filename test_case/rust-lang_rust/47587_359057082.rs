rust
    if let Some(yield_ty) = &$($mutability)* mir.yield_ty {
               self.visit_ty(yield_ty, TyContext::YieldTy(SourceInfo {
                    span: mir.span,
                    scope: ARGUMENT_VISIBILITY_SCOPE,
                }));
    }
