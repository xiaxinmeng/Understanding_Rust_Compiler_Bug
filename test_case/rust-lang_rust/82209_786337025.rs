rust
else if (matches!(self.cx.tcx.def_kind(item.def_id), DefKind::Field) && matches!(self.cx.tcx.def_kind(self.cx.tcx.parent(item.def_id).unwrap()), DefKind::Variant)) {
            self.cx.tcx.parent(item.def_id).and_then(|item_id| self.cx.tcx.parent(item_id))
} else if matches!(
            self.cx.tcx.def_kind(item.def_id),
            DefKind::AssocConst
                | DefKind::AssocFn
                | DefKind::AssocTy
                | DefKind::Field
                | DefKind::Variant
) {
