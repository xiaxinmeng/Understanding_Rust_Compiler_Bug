
664:            let upvar_tys: Vec<_> = substs.as_closure().upvar_tys().collect();
669:                &upvar_tys,
677:            let upvar_tys: Vec<_> = substs
682:            prepare_enum_metadata(cx, t, def_id, unique_type_id, usage_site_span, upvar_tys)
