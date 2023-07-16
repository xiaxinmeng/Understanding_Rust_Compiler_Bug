 rust
        let last_field = def.struct_variant().fields.last().unwrap();
        let field_ty = monomorphize::field_ty(bcx.tcx(), substs, last_field);
        let (unsized_size, unsized_align) = size_and_align_of_dst(bcx, field_ty, info);
