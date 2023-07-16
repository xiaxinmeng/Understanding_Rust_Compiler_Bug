plain
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0425]: cannot find value `def` in this scope
   --> compiler/rustc_lint/src/types.rs:707:12
    |
707 |         if def.is_unsafe_cell() {

error: unused import: `SizeSkeleton`
 --> compiler/rustc_lint/src/types.rs:9:54
  |
  |
9 | use rustc_middle::ty::layout::{IntegerExt, LayoutOf, SizeSkeleton};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`

error: unused imports: `TypeFoldable`, `TypeSuperFoldable`
   |
   |
11 | use rustc_middle::ty::{self, AdtKind, DefIdTree, Ty, TypeFoldable, TypeSuperFoldable, TypeSuperVisitable, TypeVisitable};

error[E0308]: mismatched types
   --> compiler/rustc_lint/src/types.rs:708:20
    |
