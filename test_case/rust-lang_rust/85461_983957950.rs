rust
    let mut first_covered_def_id_by_file: FxHashMap<Symbol, DefId> = FxHashMap::default();
    for &def_id in sorted_codegenned_def_ids.iter() {
        if let Some(covered_file_name) = tcx.covered_file_name(def_id) {
            // Only add files known to have unused functions
            if unused_def_ids_by_file.contains_key(covered_file_name) {
                debug!(
                    "CGU {} compiles {:?} from {}", 
                    cx.codegen_unit.name(),
                    def_id,
                    covered_file_name
                );
                first_covered_def_id_by_file.entry(*covered_file_name).or_insert(def_id);
            }
        }
    }
