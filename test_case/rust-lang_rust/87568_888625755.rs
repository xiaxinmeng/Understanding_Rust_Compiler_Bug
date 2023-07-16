plain
    Checking clippy_lints v0.1.55 (/checkout/src/tools/clippy/clippy_lints)
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/doc.rs:301:38
    |
301 |     if !cx.access_levels.is_exported(hir_id) {
    |                                      ^^^^^^ expected struct `LocalDefId`, found struct `HirId`
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/enum_variants.rs:300:83
    |
    |
300 |             if !(self.avoid_breaking_exported_api && cx.access_levels.is_exported(item.hir_id())) {
    |                                                                                   ^^^^^^^^^^^^^ expected struct `LocalDefId`, found struct `HirId`
error[E0308]: mismatched types
  --> src/tools/clippy/clippy_lints/src/exhaustive_items.rs:76:45
   |
   |
76 |             if cx.access_levels.is_exported(item.hir_id());
   |                                             ^^^^^^^^^^^^^ expected struct `LocalDefId`, found struct `HirId`
error[E0308]: mismatched types
  --> src/tools/clippy/clippy_lints/src/functions/must_use.rs:25:54
   |
   |
25 |         let is_public = cx.access_levels.is_exported(item.hir_id());
   |                                                      ^^^^^^^^^^^^^ expected struct `LocalDefId`, found struct `HirId`
error[E0308]: mismatched types
  --> src/tools/clippy/clippy_lints/src/functions/must_use.rs:46:54
   |
   |
46 |         let is_public = cx.access_levels.is_exported(item.hir_id());
   |                                                      ^^^^^^^^^^^^^ expected struct `LocalDefId`, found struct `HirId`
error[E0308]: mismatched types
  --> src/tools/clippy/clippy_lints/src/functions/must_use.rs:68:54
   |
   |
68 |         let is_public = cx.access_levels.is_exported(item.hir_id());
   |                                                      ^^^^^^^^^^^^^ expected struct `LocalDefId`, found struct `HirId`
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/functions/must_use.rs:143:42
    |
    |
143 |         || !cx.access_levels.is_exported(item_id)
    |                                          ^^^^^^^ expected struct `LocalDefId`, found struct `HirId`
error[E0308]: mismatched types
  --> src/tools/clippy/clippy_lints/src/functions/not_unsafe_ptr_arg_deref.rs:42:74
   |
   |
42 |     if unsafety == hir::Unsafety::Normal && cx.access_levels.is_exported(hir_id) {
   |                                                                          ^^^^^^ expected struct `LocalDefId`, found struct `HirId`
error[E0308]: mismatched types
  --> src/tools/clippy/clippy_lints/src/functions/result_unit_err.rs:18:54
   |
   |
18 |         let is_public = cx.access_levels.is_exported(item.hir_id());
   |                                                      ^^^^^^^^^^^^^ expected struct `LocalDefId`, found struct `HirId`
error[E0308]: mismatched types
  --> src/tools/clippy/clippy_lints/src/functions/result_unit_err.rs:28:54
   |
   |
28 |         let is_public = cx.access_levels.is_exported(item.hir_id());
   |                                                      ^^^^^^^^^^^^^ expected struct `LocalDefId`, found struct `HirId`
error[E0308]: mismatched types
  --> src/tools/clippy/clippy_lints/src/functions/result_unit_err.rs:38:54
   |
   |
38 |         let is_public = cx.access_levels.is_exported(item.hir_id());
   |                                                      ^^^^^^^^^^^^^ expected struct `LocalDefId`, found struct `HirId`
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/implicit_hasher.rs:115:42
    |
    |
115 |         if !cx.access_levels.is_exported(item.hir_id()) {
    |                                          ^^^^^^^^^^^^^ expected struct `LocalDefId`, found struct `HirId`
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/len_zero.rs:134:45
    |
    |
134 |             if cx.access_levels.is_exported(item.hir_id());
    |                                             ^^^^^^^^^^^^^ expected struct `LocalDefId`, found struct `HirId`
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/len_zero.rs:210:37
    |
    |
210 |     if cx.access_levels.is_exported(visited_trait.hir_id())
    |                                     ^^^^^^^^^^^^^^^^^^^^^^ expected struct `LocalDefId`, found struct `HirId`
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/len_zero.rs:337:30
    |
    |
337 |                 .is_exported(cx.tcx.hir().local_def_id_to_hir_id(is_empty.def_id.expect_local())) =>
    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `LocalDefId`, found struct `HirId`
error[E0308]: mismatched types
    --> src/tools/clippy/clippy_lints/src/methods/mod.rs:1896:70
     |
     |
1896 |                 if !implements_trait && cx.access_levels.is_exported(impl_item.hir_id()) {
     |                                                                      ^^^^^^^^^^^^^^^^^^ expected struct `LocalDefId`, found struct `HirId`
error[E0308]: mismatched types
    --> src/tools/clippy/clippy_lints/src/methods/mod.rs:1928:57
     |
     |
1928 |                         && cx.access_levels.is_exported(impl_item.hir_id()))
     |                                                         ^^^^^^^^^^^^^^^^^^ expected struct `LocalDefId`, found struct `HirId`
error[E0308]: mismatched types
  --> src/tools/clippy/clippy_lints/src/missing_inline.rs:90:42
   |
   |
90 |         if !cx.access_levels.is_exported(it.hir_id()) {
   |                                          ^^^^^^^^^^^ expected struct `LocalDefId`, found struct `HirId`
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/missing_inline.rs:143:42
    |
    |
143 |         if !cx.access_levels.is_exported(impl_item.hir_id()) {
    |                                          ^^^^^^^^^^^^^^^^^^ expected struct `LocalDefId`, found struct `HirId`
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/missing_inline.rs:158:73
    |
    |
158 |             if trait_def_id.is_local() && !cx.access_levels.is_exported(impl_item.hir_id()) {
    |                                                                         ^^^^^^^^^^^^^^^^^^ expected struct `LocalDefId`, found struct `HirId`
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/new_without_default.rs:103:62
    |
    |
103 | ...                   if cx.access_levels.is_reachable(id);
    |                                                        ^^ expected struct `LocalDefId`, found struct `HirId`
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/pass_by_ref_or_value.rs:134:77
    |
    |
134 |         if self.avoid_breaking_exported_api && cx.access_levels.is_exported(hir_id) {
    |                                                                             ^^^^^^ expected struct `LocalDefId`, found struct `HirId`
error[E0308]: mismatched types
  --> src/tools/clippy/clippy_lints/src/redundant_pub_crate.rs:45:46
   |
   |
45 |             if !cx.access_levels.is_exported(item.hir_id()) {
   |                                              ^^^^^^^^^^^^^ expected struct `LocalDefId`, found struct `HirId`
error[E0308]: mismatched types
  --> src/tools/clippy/clippy_lints/src/redundant_pub_crate.rs:68:64
   |
   |
68 |             self.is_exported.push(cx.access_levels.is_exported(item.hir_id()));
   |                                                                ^^^^^^^^^^^^^ expected struct `LocalDefId`, found struct `HirId`
error[E0308]: mismatched types
  --> src/tools/clippy/clippy_lints/src/unnecessary_wraps.rs:82:85
   |
   |
82 |                 if self.avoid_breaking_exported_api && cx.access_levels.is_exported(hir_id) {
   |                                                                                     ^^^^^^ expected struct `LocalDefId`, found struct `HirId`
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/upper_case_acronyms.rs:105:82
    |
    |
105 |             || (self.avoid_breaking_exported_api && cx.access_levels.is_exported(it.hir_id()))
    |                                                                                  ^^^^^^^^^^^ expected struct `LocalDefId`, found struct `HirId`
error: aborting due to 26 previous errors

For more information about this error, try `rustc --explain E0308`.
error: could not compile `clippy_lints`
