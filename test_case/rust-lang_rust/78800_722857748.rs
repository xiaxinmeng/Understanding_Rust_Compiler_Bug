rust
        cx.enter_resolver(|resolver| {
            resolver.traits_in_scope(module).iter().find(|candidate| candidate.def_id == trait_).is_some()
        })
