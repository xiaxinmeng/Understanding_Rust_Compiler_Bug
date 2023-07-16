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


error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
    --> src/tools/clippy/clippy_utils/src/higher.rs:236:38
     |
236  |                 hir::QPath::LangItem(hir::LangItem::RangeFrom, _) => Some(Range {
     |                                      ^^^^^^^^^^^^^^^^^^^^^^^^  ^ expected 3 fields, found 2
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1795:14
     |
     |
1795 |     LangItem(LangItem, Span, Option<HirId>),
     |              --------  ----  ------------- tuple variant has 3 fields
     |
help: use `_` to explicitly ignore each field
     |
236  |                 hir::QPath::LangItem(hir::LangItem::RangeFrom, _, _) => Some(Range {


error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
    --> src/tools/clippy/clippy_utils/src/higher.rs:241:38
     |
241  |                 hir::QPath::LangItem(hir::LangItem::Range, _) => Some(Range {
     |                                      ^^^^^^^^^^^^^^^^^^^^  ^ expected 3 fields, found 2
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1795:14
     |
     |
1795 |     LangItem(LangItem, Span, Option<HirId>),
     |              --------  ----  ------------- tuple variant has 3 fields
     |
help: use `_` to explicitly ignore each field
     |
241  |                 hir::QPath::LangItem(hir::LangItem::Range, _, _) => Some(Range {


error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
    --> src/tools/clippy/clippy_utils/src/higher.rs:246:38
     |
246  |                 hir::QPath::LangItem(hir::LangItem::RangeToInclusive, _) => Some(Range {
     |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^  ^ expected 3 fields, found 2
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1795:14
     |
     |
1795 |     LangItem(LangItem, Span, Option<HirId>),
     |              --------  ----  ------------- tuple variant has 3 fields
     |
help: use `_` to explicitly ignore each field
     |
246  |                 hir::QPath::LangItem(hir::LangItem::RangeToInclusive, _, _) => Some(Range {


error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
    --> src/tools/clippy/clippy_utils/src/higher.rs:251:38
     |
251  |                 hir::QPath::LangItem(hir::LangItem::RangeTo, _) => Some(Range {
     |                                      ^^^^^^^^^^^^^^^^^^^^^^  ^ expected 3 fields, found 2
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1795:14
     |
     |
1795 |     LangItem(LangItem, Span, Option<HirId>),
     |              --------  ----  ------------- tuple variant has 3 fields
     |
help: use `_` to explicitly ignore each field
     |
251  |                 hir::QPath::LangItem(hir::LangItem::RangeTo, _, _) => Some(Range {


error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
    --> src/tools/clippy/clippy_utils/src/hir_utils.rs:349:31
     |
349  |             (&QPath::LangItem(llang_item, _), &QPath::LangItem(rlang_item, _)) => llang_item == rlang_item,
     |                               ^^^^^^^^^^  ^ expected 3 fields, found 2
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1795:14
     |
     |
1795 |     LangItem(LangItem, Span, Option<HirId>),
     |              --------  ----  ------------- tuple variant has 3 fields
     |
help: use `_` to explicitly ignore each field
     |
349  |             (&QPath::LangItem(llang_item, _, _), &QPath::LangItem(rlang_item, _)) => llang_item == rlang_item,


error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
    --> src/tools/clippy/clippy_utils/src/hir_utils.rs:349:64
     |
349  |             (&QPath::LangItem(llang_item, _), &QPath::LangItem(rlang_item, _)) => llang_item == rlang_item,
     |                                                                ^^^^^^^^^^  ^ expected 3 fields, found 2
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1795:14
     |
     |
1795 |     LangItem(LangItem, Span, Option<HirId>),
     |              --------  ----  ------------- tuple variant has 3 fields
     |
help: use `_` to explicitly ignore each field
     |
349  |             (&QPath::LangItem(llang_item, _), &QPath::LangItem(rlang_item, _, _)) => llang_item == rlang_item,

    Checking idna v0.2.0
    Checking getrandom v0.2.0
    Checking dirs-sys-next v0.1.2
