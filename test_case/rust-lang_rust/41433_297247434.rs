rust
        ItemStruct(ref def, _) => {
            // Use separate constructor id for unit/tuple structs and reuse did for braced structs.
            let ctor_id = if !def.is_struct() {
                Some(tcx.hir.local_def_id(def.id()))
            } else {
                None
            };
            (AdtKind::Struct, vec![
                convert_struct_variant(tcx, ctor_id.unwrap_or(def_id), item.name,
                                       ty::VariantDiscr::Relative(0), def)
            ])
        }
