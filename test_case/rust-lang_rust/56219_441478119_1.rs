
error: unsatisfied lifetime constraints==========================>   ] 106/111: rustc_ty...  
    --> src/librustc_typeck/check/mod.rs:2788:12
     |
2786 | impl<'a, 'gcx, 'tcx> FnCtxt<'a, 'gcx, 'tcx> {
     |          ----  ---- lifetime `'tcx` defined here
     |          |
     |          lifetime `'gcx` defined here
2787 |     fn obligations_for_self_ty<'b>(&'b self, self_ty: ty::TyVid)
2788 |         -> impl Iterator<Item=ty::PolyTraitRef<'tcx>> + 'a + 'b + 'gcx + 'tcx
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'tcx` must outlive `'gcx`

error: unsatisfied lifetime constraints
    --> src/librustc_typeck/check/mod.rs:2795:9
     |
2786 |   impl<'a, 'gcx, 'tcx> FnCtxt<'a, 'gcx, 'tcx> {
     |        --  ---- lifetime `'gcx` defined here
     |        |
     |        lifetime `'a` defined here
...
2795 | /         self.fulfillment_cx
2796 | |             .borrow()
2797 | |             .pending_obligations()
2798 | |             .into_iter()
2799 | |             .filter_map(ObligationMapper(self, ty_var_root))
     | |____________________________________________________________^ returning this value requires that `'a` must outlive `'gcx`

error: unsatisfied lifetime constraints
    --> src/librustc_typeck/check/mod.rs:2795:9
     |
2786 |   impl<'a, 'gcx, 'tcx> FnCtxt<'a, 'gcx, 'tcx> {
     |        -- lifetime `'a` defined here
2787 |       fn obligations_for_self_ty<'b>(&'b self, self_ty: ty::TyVid)
     |                                  -- lifetime `'b` defined here
...
2795 | /         self.fulfillment_cx
2796 | |             .borrow()
2797 | |             .pending_obligations()
2798 | |             .into_iter()
2799 | |             .filter_map(ObligationMapper(self, ty_var_root))
     | |____________________________________________________________^ returning this value requires that `'b` must outlive `'a`
