rust
    fn find_switch_discriminant_info(
        &self,
        bb: &BasicBlockData<'tcx>,
        switch: &Terminator<'tcx>,
    ) -> Option<SwitchDiscriminantInfo<'tcx>> {
        ...
                let place_of_adt_discr_read = match bb.statements.last()?.kind {
                    StatementKind::Assign(box (_, Rvalue::Discriminant(adt_place))) => {
                        Some(adt_place)
                    }
                    _ => None,
                }?;
