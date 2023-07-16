plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0599]: no method named `super_visit_with` found for struct `ty::Ty` in the current scope
   --> compiler/rustc_middle/src/ty/diagnostics.rs:477:11
    |
477 |         t.super_visit_with(self)
    |           ^^^^^^^^^^^^^^^^ method not found in `ty::Ty<'tcx>`
   ::: compiler/rustc_middle/src/ty/mod.rs:466:1
    |
    |
466 | pub struct Ty<'tcx>(Interned<'tcx, WithStableHash<TyS<'tcx>>>);
    | --------------------------------------------------------------- method `super_visit_with` not found for this
   ::: compiler/rustc_middle/src/ty/fold.rs:229:8
    |
    |
229 |     fn super_visit_with<V: TypeVisitor<'tcx>>(&self, visitor: &mut V) -> ControlFlow<V::BreakTy>;
    |        ---------------- the method is available for `ty::Ty<'tcx>` here
    = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
    |
3   | use crate::ty::fold::TypeSuperFoldable;
3   | use crate::ty::fold::TypeSuperFoldable;
    |

error[E0599]: no method named `super_visit_with` found for struct `ty::consts::Const` in the current scope
   --> compiler/rustc_middle/src/ty/diagnostics.rs:491:11
    |
491 |         c.super_visit_with(self)
    |           ^^^^^^^^^^^^^^^^ method not found in `ty::consts::Const<'tcx>`
   ::: compiler/rustc_middle/src/ty/consts.rs:25:1
    |
    |
25  | pub struct Const<'tcx>(pub Interned<'tcx, ConstS<'tcx>>);
    | --------------------------------------------------------- method `super_visit_with` not found for this
   ::: compiler/rustc_middle/src/ty/fold.rs:229:8
    |
    |
229 |     fn super_visit_with<V: TypeVisitor<'tcx>>(&self, visitor: &mut V) -> ControlFlow<V::BreakTy>;
    |        ---------------- the method is available for `ty::consts::Const<'tcx>` here
    = help: items from traits can only be used if the trait is in scope
help: one of the expressions' fields has a method of the same name
    |
    |
491 |         c.0.0.ty.super_visit_with(self)
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
    |
3   | use crate::ty::fold::TypeSuperFoldable;
    |
