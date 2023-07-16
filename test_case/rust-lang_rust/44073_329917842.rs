
error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
  --> src/librustc_mir/build/expr/as_constant.rs:35:15
   |
35 |             = expr;
   |               ^^^^
   |
note: first, the lifetime cannot outlive the lifetime 'tcx as defined on the impl at 22:1...
  --> src/librustc_mir/build/expr/as_constant.rs:22:1
   |
22 | / impl<'a, 'gcx, 'tcx> Builder<'a, 'gcx, 'tcx> {
23 | |     /// Compile `expr`, yielding a compile-time constant. Assumes that
24 | |     /// `expr` is a valid compile-time constant!
25 | |     pub fn as_constant<M>(&mut self, expr: M) -> Constant<'tcx>
...  |
70 | |     }
71 | | }
   | |_^
note: ...so that expression is assignable (expected hair::Expr<'_>, found hair::Expr<'tcx>)
  --> src/librustc_mir/build/expr/as_constant.rs:35:15
   |
35 |             = expr;
   |               ^^^^
note: but, the lifetime must be valid for the lifetime 'gcx as defined on the impl at 22:1...
  --> src/librustc_mir/build/expr/as_constant.rs:22:1
   |
22 | / impl<'a, 'gcx, 'tcx> Builder<'a, 'gcx, 'tcx> {
23 | |     /// Compile `expr`, yielding a compile-time constant. Assumes that
24 | |     /// `expr` is a valid compile-time constant!
25 | |     pub fn as_constant<M>(&mut self, expr: M) -> Constant<'tcx>
...  |
70 | |     }
71 | | }
   | |_^
note: ...so that types are compatible (expected &build::Builder<'_, '_, '_>, found &build::Builder<'a, 'gcx, 'tcx>)
  --> src/librustc_mir/build/expr/as_constant.rs:42:32
   |
42 |                 let hir = self.hir();
   |                                ^^^
