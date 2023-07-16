plain
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0599]: no method named `visit_expr` found for reference `&IsThirPolymorphic<'a, 'tcx>` in the current scope
    |
    |
328 |                         self.visit_expr(&self.thir()[value]);
    |                              ^^^^^^^^^^ method not found in `&IsThirPolymorphic<'a, 'tcx>`
    = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
    |
11  | use crate::rustc_middle::thir::visit::Visitor;
11  | use crate::rustc_middle::thir::visit::Visitor;
    |

error[E0599]: no method named `thir` found for reference `&IsThirPolymorphic<'a, 'tcx>` in the current scope
    |
    |
328 |                         self.visit_expr(&self.thir()[value]);
    |                                               ^^^^ field, not a method
    = help: items from traits can only be used if the trait is in scope
help: remove the arguments
    |
    |
328 -                         self.visit_expr(&self.thir()[value]);
328 +                         self.visit_expr(&self.thir[value]);
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
    |
11  | use crate::rustc_middle::thir::visit::Visitor;
    |
    |

error[E0308]: mismatched types
   --> compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:327:64
    |
327 |                       thir::ExprKind::Repeat { value, count } => {
    |  ________________________________________________________________^
328 | |                         self.visit_expr(&self.thir()[value]);
329 | |                         self.is_poly |= count.has_param_types_or_consts();
    | |_____________________^ expected `bool`, found `()`

Some errors have detailed explanations: E0308, E0599.
For more information about an error, try `rustc --explain E0308`.
