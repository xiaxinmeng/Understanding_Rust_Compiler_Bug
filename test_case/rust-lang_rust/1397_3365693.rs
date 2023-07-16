
            let d = if vec::len(*ty::tag_variants(ccx.tcx, tid)) != 1u {
                let lldiscrim_gv = lookup_discriminant(bcx.fcx.lcx, vid);
                let lldiscrim = Load(bcx, lldiscrim_gv);
                lldiscrim
            } else { C_int(ccx, 0) };
