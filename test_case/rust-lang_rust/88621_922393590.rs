rust
d.variants.iter().any(|v| matches!(v.discr, VariantDiscr::Explicit(_)) && v.ctor_kind != CtorKind::Const)
