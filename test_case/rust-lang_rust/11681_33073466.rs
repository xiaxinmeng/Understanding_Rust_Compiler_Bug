
add_assignment[var](lp=$(local (id=97, name=testValue)), assignment=0, path_index=middle::borrowck::move_data::MovePathIndex(0u))
gather_loans_in_expr(expr=99u32/&Test)
guarantee_valid(borrow_id=99u32, cmt={cat_static_item id:98 m:McImmutable ty:Test}, req_mutbl=ImmutableMutability, loan_region=ReFree(middle::ty::FreeRegion{scope_id: 103u32, bound_region: BrAnon(0u)}))
guarantee_lifetime(cmt={cat_static_item id:98 m:McImmutable ty:Test}, loan_region=ReFree(103, BrAnon(0)))
guarantee_lifetime.check(cmt={cat_static_item id:98 m:McImmutable ty:Test}, loan_region=ReFree(103, BrAnon(0)))
