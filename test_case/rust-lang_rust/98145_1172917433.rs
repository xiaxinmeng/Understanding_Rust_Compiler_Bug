plain
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
error[E0412]: cannot find type `Region` in this scope
    --> compiler/rustc_middle/src/mir/mod.rs:1972:9
     |
1972 |     Ref(Region<'tcx>, BorrowKind, Place<'tcx>),
     |
help: consider importing one of these items
     |
5    | use crate::middle::resolve_lifetime::Region;
5    | use crate::middle::resolve_lifetime::Region;
     |
5    | use crate::ty::Region;
     |

    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: `syntax::Rvalue<'tcx>` doesn't implement `std::fmt::Debug`
    |
    |
177 | impl<'tcx> TypeFoldable<'tcx> for Rvalue<'tcx> {
    |            ^^^^^^^^^^^^^^^^^^ `syntax::Rvalue<'tcx>` cannot be formatted using `{:?}`
    |
    = help: the trait `std::fmt::Debug` is not implemented for `syntax::Rvalue<'tcx>`
    = note: add `#[derive(Debug)]` to `syntax::Rvalue<'tcx>` or manually `impl std::fmt::Debug for syntax::Rvalue<'tcx>`
note: required by a bound in `ty::fold::TypeFoldable`
   --> compiler/rustc_middle/src/ty/fold.rs:73:31
    |
73  | pub trait TypeFoldable<'tcx>: fmt::Debug + Clone {
    |                               ^^^^^^^^^^ required by this bound in `ty::fold::TypeFoldable`
help: consider annotating `syntax::Rvalue<'tcx>` with `#[derive(Debug)]`
919 | #[derive(Debug)]
    |

Some errors have detailed explanations: E0277, E0412.
