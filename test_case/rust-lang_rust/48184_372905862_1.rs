rust
        if let Some(found) = found {
            for ident in found {
                let ty = Ty::Path(None, ident.into());
