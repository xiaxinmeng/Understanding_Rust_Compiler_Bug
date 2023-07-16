plain
    Checking cargo_metadata v0.14.0
    Checking rustfix v0.5.1
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
    Checking clippy_lints v0.1.61 (/checkout/src/tools/clippy/clippy_lints)
error[E0599]: no variant or associated item named `I8` found for enum `rustc_hir::LangItem` in the current scope
   |
86 |                     LangItem::I8,
   |                               ^^ variant or associated item not found in `rustc_hir::LangItem`


error[E0599]: no variant or associated item named `I16` found for enum `rustc_hir::LangItem` in the current scope
   |
87 |                     LangItem::I16,
   |                               ^^^ variant or associated item not found in `rustc_hir::LangItem`


error[E0599]: no variant or associated item named `I32` found for enum `rustc_hir::LangItem` in the current scope
   |
88 |                     LangItem::I32,
   |                               ^^^ variant or associated item not found in `rustc_hir::LangItem`


error[E0599]: no variant or associated item named `I64` found for enum `rustc_hir::LangItem` in the current scope
   |
89 |                     LangItem::I64,
   |                               ^^^ variant or associated item not found in `rustc_hir::LangItem`


error[E0599]: no variant or associated item named `Isize` found for enum `rustc_hir::LangItem` in the current scope
   |
90 |                     LangItem::Isize
   |                               ^^^^^ variant or associated item not found in `rustc_hir::LangItem`


error[E0599]: no method named `slice_impl` found for reference `&LanguageItems` in the current scope
    |
427 | ...                   cx.tcx.lang_items().slice_impl()
427 | ...                   cx.tcx.lang_items().slice_impl()
    |                                           ^^^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `str_impl` found for reference `&LanguageItems` in the current scope
    |
    |
432 | ...                   cx.tcx.lang_items().str_impl()
    |                                           ^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `slice_alloc_impl` found for reference `&LanguageItems` in the current scope
   |
   |
54 |                 .map(|impl_did| Some(impl_did) == cx.tcx.lang_items().slice_alloc_impl())
   |                                                                       ^^^^^^^^^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `slice_impl` found for reference `&LanguageItems` in the current scope
  --> src/tools/clippy/clippy_lints/src/methods/suspicious_splitn.rs:16:23
   |
16 |         if lang_items.slice_impl() == Some(impl_id) || lang_items.str_impl() == Some(impl_id);
   |                       ^^^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `str_impl` found for reference `&LanguageItems` in the current scope
  --> src/tools/clippy/clippy_lints/src/methods/suspicious_splitn.rs:16:67
   |
16 |         if lang_items.slice_impl() == Some(impl_id) || lang_items.str_impl() == Some(impl_id);
   |                                                                   ^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `slice_impl` found for reference `&LanguageItems` in the current scope
  --> src/tools/clippy/clippy_lints/src/methods/suspicious_splitn.rs:31:31
   |
31 |                 if lang_items.slice_impl() == Some(impl_id) {
   |                               ^^^^^^^^^^ method not found in `&LanguageItems`
error[E0277]: the size for values of type `str` cannot be known at compilation time
  --> src/tools/clippy/clippy_lints/src/methods/suspicious_splitn.rs:26:18
   |
   |
26 |             let (msg, note_msg) = if count == 0 {
   |
   = help: the trait `std::marker::Sized` is not implemented for `str`
   = note: all local variables must have a statically known size
   = help: unsized locals are gated as an unstable feature
