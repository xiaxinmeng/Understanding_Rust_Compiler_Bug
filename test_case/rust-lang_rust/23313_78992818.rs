 rust
    let left_ty = if real_pat.id == DUMMY_NODE_ID {
        ty::mk_nil(cx.tcx)
    } else {
        ty::pat_ty(cx.tcx, &*real_pat)
    };
