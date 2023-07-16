
               // Then try to borrow to a slice *and* borrow a pointer.
                // NB: we do not try to autoref to a mutable pointer. That would
                // be creating a pointer to a temporary pointer (the borrowed slice),
                // so any update the callee makes to it can't be observed.
                self.search_for_some_kind_of_autorefd_method(
                    AutoBorrowVecRef, autoderefs, [m_mut,m_const,m_imm], /* <-- note: try all 3 */
                    |m,r| {
                        let slice_ty = ty::mk_evec(tcx,
                                                   {ty:mt.ty, mutbl:m}, /* <-- note: mutbl: m */
                                                   vstore_slice(r));
                        ty::mk_rptr(tcx, r, {ty:slice_ty, mutbl:m_imm}) /* <-- note: also m_imm */
                    })
