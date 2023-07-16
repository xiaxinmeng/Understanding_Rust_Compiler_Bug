plain
   |
79 |     self, abort, const_eval_select, is_aligned_and_not_null, is_nonoverlapping,
   |                                                              ^^^^^^^^^^^^^^^^^
   |                                                              |
   |                                                              no `is_nonoverlapping` in `intrinsics`
   |                                                              help: a similar name exists in the module: `copy_nonoverlapping`
error: unused import: `const_eval_select`
  --> library/core/src/ptr/mod.rs:79:18
   |
79 |     self, abort, const_eval_select, is_aligned_and_not_null, is_nonoverlapping,
79 |     self, abort, const_eval_select, is_aligned_and_not_null, is_nonoverlapping,
   |                  ^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
For more information about this error, try `rustc --explain E0432`.
error: could not compile `core` due to 2 previous errors
Build completed unsuccessfully in 0:01:10
