
ItemImpl(_, _, defaultness, ref generics, ..) => {
    if defaultness.is_default()  {
        opt_self = Some(ty::TypeParameterDef {
            index: 0,
            name: keywords::SelfType.name(),
            def_id: tcx.hir.local_def_id(item.id),
            has_default: false,
            object_lifetime_default: rl::Set1::Empty,
            pure_wrt_drop: false,
            synthetic: None,
        });

        allow_defaults = true;
    }
    (generics, None)
} 
