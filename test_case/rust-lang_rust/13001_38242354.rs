 rust
        // A function pointer is called without the declaration
        // available, so we have to apply any attributes with ABI
        // implications directly to the call instruction. Right now,
        // the only attribute we need to worry about is `sret`.
        let mut attrs = Vec::new();
        if type_of::return_uses_outptr(ccx, ret_ty) {
            attrs.push((1, StructRetAttribute));
        }

        // The `noalias` attribute on the return value is useful to a
        // function ptr caller.
        match ty::get(ret_ty).sty {
            // `~` pointer return values never alias because ownership
            // is transferred
            ty::ty_uniq(..) | ty::ty_vec(_, ty::vstore_uniq) => {
                attrs.push((0, NoAliasAttribute));
            }
            _ => {}
        }
