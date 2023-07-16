plain
    Checking rustc_ast_pretty v0.0.0 (/checkout/compiler/rustc_ast_pretty)
    Checking rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
    Checking chalk-solve v0.55.0
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
error[E0277]: the trait bound `Enumerated<DefIndex, std::slice::Iter<'_, DefKey>>: ExactSizeIterator` is not satisfied
  --> compiler/rustc_hir/src/definitions.rs:90:10
   |
90 |     ) -> impl Iterator<Item = (DefIndex, &DefKey, &DefPathHash)> + ExactSizeIterator + '_ {
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `ExactSizeIterator` is not implemented for `Enumerated<DefIndex, std::slice::Iter<'_, DefKey>>`
   |
   = note: required because of the requirements on the impl of `ExactSizeIterator` for `std::iter::Map<Enumerated<DefIndex, std::slice::Iter<'_, DefKey>>, [closure@compiler/rustc_hir/src/definitions.rs:93:18: 93:80]>`
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_hir` due to previous error
warning: build failed, waiting for other jobs to finish...
