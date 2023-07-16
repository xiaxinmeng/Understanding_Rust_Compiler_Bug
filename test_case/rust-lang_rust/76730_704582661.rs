rust
                let (ident, is_simple_parameter) = match parameter.pat.kind {
                    hir::PatKind::Binding(
                        hir::BindingAnnotation::Unannotated | hir::BindingAnnotation::Mutable,
                        _,
                        ident,
                        _,
                    ) => (ident, true),
                    hir::PatKind::Binding(_, _, ident, _) => (ident, false),
