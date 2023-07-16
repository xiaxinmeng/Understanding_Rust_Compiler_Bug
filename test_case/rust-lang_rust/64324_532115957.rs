rust
// The query
fn symbol_name(tcx: TyCtxt<'tcx>, instance: Instance<'tcx>, instantiating_crate: Option<CrateNum>) -> InternedString {
    // ...
}

// Helper method
impl TyCtxt {
    // What an awesome method name!
    fn symbol_name_as_used_locally(self, instance: Instance) -> InternedString {
        // Do the deduction of the instantiating crate here.
        // This is moved out of the query provider above.
        let is_generic = substs.non_erasable_generics().next().is_some();
        let avoid_cross_crate_conflicts =
            // If this is an instance of a generic function, we also hash in
            // the ID of the instantiating crate. This avoids symbol conflicts
            // in case the same instances is emitted in two crates of the same
            // project.
            is_generic ||

            // If we're dealing with an instance of a function that's inlined from
            // another crate but we're marking it as globally shared to our
            // compliation (aka we're not making an internal copy in each of our
            // codegen units) then this symbol may become an exported (but hidden
            // visibility) symbol. This means that multiple crates may do the same
            // and we want to be sure to avoid any symbol conflicts here.
            match MonoItem::Fn(instance).instantiation_mode(tcx) {
                InstantiationMode::GloballyShared { may_conflict: true } => true,
                _ => false,
            };

        let instantiating_crate = if avoid_cross_crate_conflicts {
            Some(if is_generic {
                if !def_id.is_local() && tcx.sess.opts.share_generics() {
                    // If we are re-using a monomorphization from another crate,
                    // we have to compute the symbol hash accordingly.
                    let upstream_monomorphizations = tcx.upstream_monomorphizations_for(def_id);

                    upstream_monomorphizations
                        .and_then(|monos| monos.get(&substs).cloned())
                        .unwrap_or(LOCAL_CRATE)
                } else {
                    LOCAL_CRATE
                }
            } else {
                LOCAL_CRATE
            })
        } else {
            None
        };

        self.symbol_name(instance, instantiating_crate)
    }
}


/// and then in back/linker.rs

fn exported_symbols(tcx: TyCtxt<'_>, crate_type: CrateType) -> Vec<String> {

    // ...

    // For each dependency that we are linking to statically ... 
    if *dep_format == Linkage::Static { 
        // ... we add its symbol list to our export list. 
        for &(symbol, level) in tcx.exported_symbols(cnum).iter() { 
            if level.is_below_threshold(export_threshold) { 
                symbols.push(tcx.symbol_name(symbol, cnum).to_string());
            } 
        } 
    }

    // ...
}
