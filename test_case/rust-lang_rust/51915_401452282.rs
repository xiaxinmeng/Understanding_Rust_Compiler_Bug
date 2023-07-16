
- librustc_passes: 1 error
error[E0502]: cannot borrow `*self.tables` as immutable because it is also borrowed as mutable
   --> librustc_passes/rvalue_promotion.rs:206:76
    |
206 |         euv::ExprUseVisitor::new(self, tcx, param_env, &region_scope_tree, self.tables, None)
    |         -------------------------------------------------------------------^^^^^^^^^^^-------
    |         |                        |                                         |
    |         |                        |                                         immutable borrow occurs here
    |         |                        mutable borrow occurs here
| borrow later used here
