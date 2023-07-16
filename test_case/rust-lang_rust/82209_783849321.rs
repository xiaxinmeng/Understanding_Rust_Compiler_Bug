rust
} else if matches!(
            self.cx.tcx.def_kind(item.def_id),
            DefKind::AssocConst
                | DefKind::AssocFn
                | DefKind::AssocTy
                | DefKind::Variant
                | DefKind::Field
                | DefKind::Ctor(_, _)
        ) {
            self.cx.tcx.parent(item.def_id)
