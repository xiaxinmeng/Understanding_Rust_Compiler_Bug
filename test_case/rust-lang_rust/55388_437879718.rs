   
let substs = tcx.mk_substs([
        Kind::from(self_ty),
        Kind::from(sig.inputs()[0]),
    ].iter().cloned());
