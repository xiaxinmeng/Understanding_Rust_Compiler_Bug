plain
    Checking cargo_metadata v0.14.0
    Checking rustfix v0.6.1
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
    Checking clippy_lints v0.1.64 (/checkout/src/tools/clippy/clippy_lints)
error[E0425]: cannot find function `is_type_lang_item` in this scope
  --> src/tools/clippy/clippy_lints/src/bytes_count_to_len.rs:55:16
   |
55 |             if is_type_lang_item(cx, ty, sym::String) || ty.kind() == &ty::Str;
   |
help: consider importing this function
   |
1  | use clippy_utils::ty::is_type_lang_item;
1  | use clippy_utils::ty::is_type_lang_item;
   |

error[E0425]: cannot find function `is_type_lang_item` in this scope
   |
   |
38 |     is_type_lang_item(cx, cx.typeck_results().expr_ty(e).peel_refs(), sym::String)
   |
help: consider importing this function
   |
1  | use clippy_utils::ty::is_type_lang_item;
1  | use clippy_utils::ty::is_type_lang_item;
   |

error[E0425]: cannot find function `is_type_lang_item` in this scope
    |
    |
102 |     is_type_lang_item(cx, ty, sym::String) || is_type_diagnostic_item(cx, ty, sym::str)
    |
help: consider importing this function
    |
1   | use clippy_utils::ty::is_type_lang_item;
1   | use clippy_utils::ty::is_type_lang_item;
    |

error[E0425]: cannot find function `is_type_lang_item` in this scope
   --> src/tools/clippy/clippy_lints/src/inherent_to_string.rs:108:16
    |
108 |             if is_type_lang_item(cx, return_ty(cx, impl_item.hir_id()), sym::String);
    |
help: consider importing this function
    |
1   | use clippy_utils::ty::is_type_lang_item;
1   | use clippy_utils::ty::is_type_lang_item;
    |

error[E0425]: cannot find function `is_type_lang_item` in this scope
    |
    |
139 |         && is_type_lang_item(cx, ty, sym::String)
    |
help: consider importing this function
    |
1   | use clippy_utils::ty::is_type_lang_item;
1   | use clippy_utils::ty::is_type_lang_item;
    |

error[E0425]: cannot find function `is_type_lang_item` in this scope
   |
   |
67 |             if is_type_lang_item(self.cx, ty, sym::String) || ty.kind() == &ty::Str {
   |
help: consider importing this function
   |
1  | use clippy_utils::ty::is_type_lang_item;
1  | use clippy_utils::ty::is_type_lang_item;
   |

error[E0425]: cannot find function `is_type_lang_item` in this scope
  --> src/tools/clippy/clippy_lints/src/methods/bytes_nth.rs:20:15
   |
20 |     } else if is_type_lang_item(cx, ty, sym::String) {
   |
help: consider importing this function
   |
1  | use clippy_utils::ty::is_type_lang_item;
1  | use clippy_utils::ty::is_type_lang_item;
   |

error[E0425]: cannot find function `is_type_lang_item` in this scope
   |
   |
39 | ...                   || is_type_lang_item(cx, base_type, sym::String)
   |
help: consider importing this function
   |
1  | use clippy_utils::ty::is_type_lang_item;
1  | use clippy_utils::ty::is_type_lang_item;
   |

error[E0425]: cannot find function `is_type_lang_item` in this scope
   |
   |
57 |         if is_type_lang_item(cx, arg_ty, sym::String) {
   |
help: consider importing this function
   |
1  | use clippy_utils::ty::is_type_lang_item;
1  | use clippy_utils::ty::is_type_lang_item;
   |

error[E0425]: cannot find function `is_type_lang_item` in this scope
  --> src/tools/clippy/clippy_lints/src/methods/inefficient_to_string.rs:63:8
   |
63 |     if is_type_lang_item(cx, ty, sym::String) {
   |
help: consider importing this function
   |
1  | use clippy_utils::ty::is_type_lang_item;
1  | use clippy_utils::ty::is_type_lang_item;
   |

error[E0425]: cannot find function `is_type_lang_item` in this scope
   |
   |
19 |     if !(ty.is_str() || is_type_lang_item(cx, ty, sym::String)) {
   |
help: consider importing this function
   |
1  | use clippy_utils::ty::is_type_lang_item;
1  | use clippy_utils::ty::is_type_lang_item;
   |

error[E0425]: cannot find function `is_type_lang_item` in this scope
   --> src/tools/clippy/clippy_lints/src/methods/search_is_some.rs:108:16
    |
108 |             if is_type_lang_item(cx, self_ty, sym::String) {
    |
help: consider importing this function
    |
1   | use clippy_utils::ty::is_type_lang_item;
1   | use clippy_utils::ty::is_type_lang_item;
    |

error[E0425]: cannot find function `is_type_lang_item` in this scope
  --> src/tools/clippy/clippy_lints/src/methods/string_extend_chars.rs:20:9
   |
20 |     if !is_type_lang_item(cx, obj_ty, sym::String) {
   |
help: consider importing this function
   |
1  | use clippy_utils::ty::is_type_lang_item;
1  | use clippy_utils::ty::is_type_lang_item;
   |

error[E0425]: cannot find function `is_type_lang_item` in this scope
  --> src/tools/clippy/clippy_lints/src/methods/string_extend_chars.rs:28:19
   |
28 |         } else if is_type_lang_item(cx, self_ty, sym::String) {
   |
help: consider importing this function
   |
1  | use clippy_utils::ty::is_type_lang_item;
1  | use clippy_utils::ty::is_type_lang_item;
   |

error[E0425]: cannot find function `is_type_lang_item` in this scope
  --> src/tools/clippy/clippy_lints/src/methods/unnecessary_join.rs:24:12
   |
24 |         if is_type_lang_item(cx, *slice, sym::String);
   |
help: consider importing this function
   |
1  | use clippy_utils::ty::is_type_lang_item;
1  | use clippy_utils::ty::is_type_lang_item;
   |

error[E0425]: cannot find function `is_type_lang_item` in this scope
   --> src/tools/clippy/clippy_lints/src/needless_pass_by_value.rs:264:28
    |
264 |                         if is_type_lang_item(cx, ty, sym::String) {
    |
help: consider importing this function
    |
1   | use clippy_utils::ty::is_type_lang_item;
1   | use clippy_utils::ty::is_type_lang_item;
    |

error[E0425]: cannot find function `is_type_lang_item` in this scope
    |
    |
128 |                     && is_type_lang_item(cx, arg_ty, sym::String));
    |
help: consider importing this function
    |
1   | use clippy_utils::ty::is_type_lang_item;
1   | use clippy_utils::ty::is_type_lang_item;
    |

error[E0425]: cannot find function `is_type_lang_item` in this scope
   |
   |
75 |                 } else if is_type_lang_item(cx, ty, sym::String) {
   |
help: consider importing this function
   |
1  | use clippy_utils::ty::is_type_lang_item;
1  | use clippy_utils::ty::is_type_lang_item;
   |

error[E0425]: cannot find function `is_type_lang_item` in this scope
    |
    |
187 |                 if matches!(e_ty.kind(), ty::Str) || is_type_lang_item(cx, e_ty, sym::String) {
    |
help: consider importing this function
    |
1   | use clippy_utils::ty::is_type_lang_item;
1   | use clippy_utils::ty::is_type_lang_item;
    |

error[E0425]: cannot find function `is_type_lang_item` in this scope
    |
    |
202 |     is_type_lang_item(cx, cx.typeck_results().expr_ty(e).peel_refs(), sym::String)
    |
help: consider importing this function
    |
1   | use clippy_utils::ty::is_type_lang_item;
1   | use clippy_utils::ty::is_type_lang_item;
    |

error[E0425]: cannot find function `is_type_lang_item` in this scope
    |
    |
439 |             if is_type_lang_item(cx, ty, sym::String);
    |
help: consider importing this function
    |
1   | use clippy_utils::ty::is_type_lang_item;
1   | use clippy_utils::ty::is_type_lang_item;
    |

error[E0425]: cannot find function `is_type_lang_item` in this scope
   |
   |
64 |                         if is_type_lang_item(cx, inner_expr_type, sym::String);
   |
help: consider importing this function
   |
1  | use clippy_utils::ty::is_type_lang_item;
---
  |
3 | use clippy_utils::ty::is_type_diagnostic_item;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `symbol::sym`
 --> src/tools/clippy/clippy_lints/src/case_sensitive_file_extension_comparisons.rs:8:39
  |
  |
8 | use rustc_span::{source_map::Spanned, symbol::sym, Span};

error: unused import: `clippy_utils::ty::is_type_diagnostic_item`
 --> src/tools/clippy/clippy_lints/src/format_push_string.rs:2:5
  |
  |
2 | use clippy_utils::ty::is_type_diagnostic_item;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: unused import: `is_type_diagnostic_item`
 --> src/tools/clippy/clippy_lints/src/inherent_to_string.rs:2:42
  |
2 | use clippy_utils::ty::{implements_trait, is_type_diagnostic_item};

error: unused import: `clippy_utils::ty::is_type_diagnostic_item`
 --> src/tools/clippy/clippy_lints/src/matches/match_str_case_mismatch.rs:2:5
  |
---

error: unused import: `is_type_diagnostic_item`
 --> src/tools/clippy/clippy_lints/src/methods/inefficient_to_string.rs:3:24
  |
3 | use clippy_utils::ty::{is_type_diagnostic_item, walk_ptrs_ty_depth};

error: unused import: `is_type_diagnostic_item`
 --> src/tools/clippy/clippy_lints/src/methods/manual_str_repeat.rs:4:24
  |
  |
4 | use clippy_utils::ty::{is_type_diagnostic_item, is_type_lang_item, match_type};

error: unused import: `clippy_utils::ty::is_type_diagnostic_item`
 --> src/tools/clippy/clippy_lints/src/methods/no_effect_replace.rs:2:5
  |
---

error: unused import: `ty::is_type_diagnostic_item`
 --> src/tools/clippy/clippy_lints/src/methods/unnecessary_join.rs:1:53
  |
1 | use clippy_utils::{diagnostics::span_lint_and_sugg, ty::is_type_diagnostic_item};

error: unused import: `clippy_utils::ty::is_type_diagnostic_item`
 --> src/tools/clippy/clippy_lints/src/repeat_once.rs:4:5
  |
---

error: unused import: `ty::is_type_diagnostic_item`
 --> src/tools/clippy/clippy_lints/src/unnecessary_owned_empty_strings.rs:1:53
  |
1 | use clippy_utils::{diagnostics::span_lint_and_sugg, ty::is_type_diagnostic_item};

error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/methods/manual_str_repeat.rs:35:38
    |
    |
35  |         if is_type_lang_item(cx, ty, sym::String)
    |            -----------------         ^^^^^^^^^^^ expected enum `rustc_hir::LangItem`, found struct `Symbol`
    |            arguments to this function are incorrect
    |
note: function defined here
   --> /checkout/src/tools/clippy/clippy_utils/src/ty.rs:322:8
   --> /checkout/src/tools/clippy/clippy_utils/src/ty.rs:322:8
    |
322 | pub fn is_type_lang_item(cx: &LateContext<'_>, ty: Ty<'_>, lang_item: hir::LangItem) -> bool {

error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/methods/manual_str_repeat.rs:43:55
    |
    |
43  |             (ty.is_str() || is_type_lang_item(cx, ty, sym::String)).then(|| RepeatKind::String)
    |                             -----------------         ^^^^^^^^^^^ expected enum `rustc_hir::LangItem`, found struct `Symbol`
    |                             arguments to this function are incorrect
    |
note: function defined here
   --> /checkout/src/tools/clippy/clippy_utils/src/ty.rs:322:8
   --> /checkout/src/tools/clippy/clippy_utils/src/ty.rs:322:8
    |
322 | pub fn is_type_lang_item(cx: &LateContext<'_>, ty: Ty<'_>, lang_item: hir::LangItem) -> bool {

error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/methods/manual_str_repeat.rs:58:77
    |
    |
58  |         if is_type_lang_item(cx, cx.typeck_results().expr_ty(collect_expr), sym::String);
    |            ----------------- arguments to this function are incorrect       ^^^^^^^^^^^ expected enum `rustc_hir::LangItem`, found struct `Symbol`
note: function defined here
   --> /checkout/src/tools/clippy/clippy_utils/src/ty.rs:322:8
    |
    |
322 | pub fn is_type_lang_item(cx: &LateContext<'_>, ty: Ty<'_>, lang_item: hir::LangItem) -> bool {

error[E0308]: mismatched types
  --> src/tools/clippy/clippy_lints/src/types/box_collection.rs:45:69
   |
   |
45 |         .or_else(|| cx.tcx.lang_items().string().filter(|did| id == did))
   |                                                                     ^^^ expected struct `DefId`, found `&DefId`
help: consider dereferencing the borrow
   |
   |
45 |         .or_else(|| cx.tcx.lang_items().string().filter(|did| id == *did))

error[E0308]: mismatched types
  --> src/tools/clippy/clippy_lints/src/types/box_collection.rs:45:21
   |
   |
39 | fn get_std_collection(cx: &LateContext<'_>, qpath: &QPath<'_>) -> Option<Symbol> {
   |                                                                   -------------- expected `std::option::Option<Symbol>` because of return type
...
45 |         .or_else(|| cx.tcx.lang_items().string().filter(|did| id == did))
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Symbol`, found struct `DefId`
   = note: expected enum `std::option::Option<Symbol>`
   = note: expected enum `std::option::Option<Symbol>`
              found enum `std::option::Option<DefId>`
Some errors have detailed explanations: E0308, E0425.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `clippy_lints` due to 42 previous errors
Build completed unsuccessfully in 0:04:06
