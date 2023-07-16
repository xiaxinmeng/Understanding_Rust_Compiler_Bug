plain
   Compiling clippy v0.1.58 (/checkout/src/tools/clippy)
    Checking bstr v0.2.13
    Checking quote v1.0.7
   Compiling libz-sys v1.1.3
error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
    --> src/tools/clippy/clippy_utils/src/higher.rs:221:62
     |
221  |                     hir::ExprKind::Path(hir::QPath::LangItem(hir::LangItem::RangeInclusiveNew, _))
     |                                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^  ^ expected 3 fields, found 2
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1795:14
     |
     |
1795 |     LangItem(LangItem, Span, Option<HirId>),
     |              --------  ----  ------------- tuple variant has 3 fields
     |
help: use `_` to explicitly ignore each field
     |
221  |                     hir::ExprKind::Path(hir::QPath::LangItem(hir::LangItem::RangeInclusiveNew, _, _))


error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
    --> src/tools/clippy/clippy_utils/src/higher.rs:231:38
     |
231  |                 hir::QPath::LangItem(hir::LangItem::RangeFull, _) => Some(Range {
     |                                      ^^^^^^^^^^^^^^^^^^^^^^^^  ^ expected 3 fields, found 2
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1795:14
     |
     |
1795 |     LangItem(LangItem, Span, Option<HirId>),
     |              --------  ----  ------------- tuple variant has 3 fields
     |
help: use `_` to explicitly ignore each field
     |
231  |                 hir::QPath::LangItem(hir::LangItem::RangeFull, _, _) => Some(Range {

    Checking idna v0.2.0
    Checking getrandom v0.2.0
    Checking dirs-sys-next v0.1.2
