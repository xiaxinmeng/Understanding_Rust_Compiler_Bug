
    let llblobptr =
        if is_degen {
            fcx.llretptr
        } else {
            let lltagptr =
                PointerCast(bcx, fcx.llretptr, T_opaque_tag_ptr(ccx));
            let lldiscrimptr = GEPi(bcx, lltagptr, [0, 0]);
            Store(bcx, C_int(ccx, index), lldiscrimptr);
            GEPi(bcx, lltagptr, [0, 1])
        };
