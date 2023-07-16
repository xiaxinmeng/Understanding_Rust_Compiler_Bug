plain
    Checking url v2.1.1
    Checking clippy_utils v0.1.53 (/checkout/src/tools/clippy/clippy_utils)
    Checking cargo_metadata v0.12.0
    Checking clippy_lints v0.1.53 (/checkout/src/tools/clippy/clippy_lints)
error[E0599]: no variant or associated item named `TryIntoResult` found for enum `rustc_hir::LangItem` in the current scope
   --> src/tools/clippy/clippy_lints/src/needless_question_mark.rs:150:57
    |
150 |         if let ExprKind::Path(QPath::LangItem(LangItem::TryIntoResult, _)) = &called.kind;
    |                                                         ^^^^^^^^^^^^^ variant or associated item not found in `rustc_hir::LangItem`

error[E0599]: no variant or associated item named `TryIntoResult` found for enum `rustc_hir::LangItem` in the current scope
  --> src/tools/clippy/clippy_lints/src/try_err.rs:67:67
   |
67 |             if matches!(match_fun_path, QPath::LangItem(LangItem::TryIntoResult, _));
   |                                                                   ^^^^^^^^^^^^^ variant or associated item not found in `rustc_hir::LangItem`

error[E0599]: no variant or associated item named `TryIntoResult` found for enum `rustc_hir::LangItem` in the current scope
   |
   |
48 |                         hir::ExprKind::Path(hir::QPath::LangItem(hir::LangItem::TryIntoResult, _))
   |                                                                                 ^^^^^^^^^^^^^ variant or associated item not found in `rustc_hir::LangItem`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0599`.
error: could not compile `clippy_lints`
