plain
[RUSTC-TIMING] rustc_attr test:false 4.478
[RUSTC-TIMING] rustc_query_system test:false 5.365
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
[RUSTC-TIMING] rustc_errors test:false 9.311
error[E0271]: type mismatch resolving `<F as TypeFolder<'tcx>>::Error == !`
   --> compiler/rustc_middle/src/ty/structural_impls.rs:744:32
    |
744 |             let folded = owned.fold_with(folder)?;
    |                                ^^^^^^^^^ expected `!`, found associated type
    = note:         expected type `!`
    = note:         expected type `!`
            found associated type `<F as TypeFolder<'tcx>>::Error`
    = help: consider constraining the associated type `<F as TypeFolder<'tcx>>::Error` to `!`
    = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
note: required by a bound in `ty::fold::TypeFoldable::fold_with`
   --> compiler/rustc_middle/src/ty/fold.rs:62:38
    |
62  |     fn fold_with<F: TypeFolder<'tcx, Error = !>>(self, folder: &mut F) -> Self {
    |                                      ^^^^^^^^^ required by this bound in `ty::fold::TypeFoldable::fold_with`
error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> compiler/rustc_middle/src/ty/structural_impls.rs:744:26
    |
    |
744 |             let folded = owned.fold_with(folder)?;
    |                          ^^^^^^^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `T`
help: consider further restricting this bound
    |
    |
734 | impl<'tcx, T: TypeFoldable<'tcx> + std::ops::Try> TypeFoldable<'tcx> for Rc<T> {


error[E0271]: type mismatch resolving `<F as TypeFolder<'tcx>>::Error == !`
   --> compiler/rustc_middle/src/ty/structural_impls.rs:765:32
    |
765 |             let folded = owned.fold_with(folder)?;
    |                                ^^^^^^^^^ expected `!`, found associated type
    = note:         expected type `!`
    = note:         expected type `!`
            found associated type `<F as TypeFolder<'tcx>>::Error`
    = help: consider constraining the associated type `<F as TypeFolder<'tcx>>::Error` to `!`
    = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
note: required by a bound in `ty::fold::TypeFoldable::fold_with`
   --> compiler/rustc_middle/src/ty/fold.rs:62:38
    |
62  |     fn fold_with<F: TypeFolder<'tcx, Error = !>>(self, folder: &mut F) -> Self {
    |                                      ^^^^^^^^^ required by this bound in `ty::fold::TypeFoldable::fold_with`
error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> compiler/rustc_middle/src/ty/structural_impls.rs:765:26
    |
    |
765 |             let folded = owned.fold_with(folder)?;
    |                          ^^^^^^^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `T`
help: consider further restricting this bound
    |
    |
755 | impl<'tcx, T: TypeFoldable<'tcx> + std::ops::Try> TypeFoldable<'tcx> for Arc<T> {

[RUSTC-TIMING] rustc_llvm test:false 0.292
Some errors have detailed explanations: E0271, E0277.
For more information about an error, try `rustc --explain E0271`.
