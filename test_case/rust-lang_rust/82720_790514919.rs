plain
    |
707 | ) -> Ty<'_> {
    |             - unclosed delimiter
...
730 |             if !ty.references_error() { {
    |                                       - this delimiter might not be properly closed...
740 |         }
740 |         }
    |         - ...as it matches this but it has different indentation
762 | }
    |   ^


error: expected one of `=>`, `@`, `if`, or `|`, found `.`
   --> compiler/rustc_typeck/src/collect/type_of.rs:744:8
    |
744 |     tcx.fold_regions(ty, &mut false, |r, _| match r {
    |        ^ expected one of `=>`, `@`, `if`, or `|`

error[E0425]: cannot find function `check_feature_inherent_assoc_ty` in this scope
   --> compiler/rustc_typeck/src/collect/type_of.rs:296:21
    |
296 |                     check_feature_inherent_assoc_ty(tcx, item.span);

error[E0308]: mismatched types
   --> compiler/rustc_typeck/src/collect/type_of.rs:707:6
    |
    |
701 | fn infer_placeholder_type(
    |    ---------------------- implicitly returns `()` as its body has no tail or `return` expression
707 | ) -> Ty<'_> {
707 | ) -> Ty<'_> {
    |      ^^^^^^ expected `&TyS<'_>`, found `()`
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0308, E0425.
For more information about an error, try `rustc --explain E0308`.
