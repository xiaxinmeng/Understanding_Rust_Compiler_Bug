plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0053]: method `visit_const` has an incompatible type for trait
    --> compiler/rustc_middle/src/mir/pretty.rs:689:38
     |
689  |         fn visit_const(&mut self, c: &&'tcx ty::Const<'tcx>, _loc: Location) {
     |                                      |
     |                                      |
     |                                      expected struct `ty::consts::Const`, found `&&'tcx ty::consts::Const<'tcx>`
     |                                      help: change the parameter type to match the trait: `ty::consts::Const<'tcx>`
note: type in trait
    --> compiler/rustc_middle/src/mir/visit.rs:203:56
     |
     |
68   | / macro_rules! make_mir_visitor {
69   | |     ($visitor_trait_name:ident, $($mutability:ident)?) => {
70   | |         pub trait $visitor_trait_name<'tcx> {
71   | |             // Override these, and call `self.super_xxx` to revert back to the
...    |
203  | |                            constant: $(& $mutability)? ty::Const<'tcx>,
...    |
935  | |     }
936  | | }
936  | | }
     | |_- in this expansion of `make_mir_visitor!`
...
1090 |   make_mir_visitor!(Visitor,);
     |   --------------------------- in this macro invocation
     = note: expected fn pointer `fn(&mut CollectAllocIds, ty::consts::Const<'tcx>, mir::Location)`
                found fn pointer `fn(&mut CollectAllocIds, &&'tcx ty::consts::Const<'tcx>, mir::Location)`

error[E0615]: attempted to take value of method `val` on type `&&'tcx ty::consts::Const<'tcx>`
   --> compiler/rustc_middle/src/mir/pretty.rs:690:50
    |
690 |             if let ty::ConstKind::Value(val) = c.val {
    |
help: use parentheses to call the method
    |
    |
690 |             if let ty::ConstKind::Value(val) = c.val() {

error[E0308]: mismatched types
   --> compiler/rustc_middle/src/mir/pretty.rs:697:57
    |
    |
697 |                 ConstantKind::Ty(c) => self.visit_const(&c, loc),
    |                                                         ^^ expected struct `ty::consts::Const`, found `&ty::consts::Const<'_>`
help: consider removing the borrow
    |
    |
697 -                 ConstantKind::Ty(c) => self.visit_const(&c, loc),
697 +                 ConstantKind::Ty(c) => self.visit_const(c, loc),

Some errors have detailed explanations: E0053, E0308, E0615.
For more information about an error, try `rustc --explain E0053`.
error: could not compile `rustc_middle` due to 3 previous errors
