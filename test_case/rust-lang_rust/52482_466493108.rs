rust
        hir::ExprKind::Cast(ref source, ref cast_ty) => {
            // Check for a user-given type annotation on this `cast`
            let user_provided_types = cx.tables.user_provided_types();
            let user_ty = user_provided_types.get(cast_ty.hir_id);

            debug!(
                "cast({:?}) has ty w/ hir_id {:?} and user provided ty {:?}",
                expr,
                cast_ty.hir_id,
                user_ty,
            );
            
            let source_ty = cx.tables().expr_ty(source);
            let cast = if source_ty == expr_ty {
                // Convert the lexpr to a vexpr.
                ExprKind::Use { source: source.to_ref() }
            } 
