
// Compute a map from each struct/enum/union S to the **explicit**
// outlives predicates (`T: 'a`, `'a: 'b`) that the user wrote.
// Typically there won't be many of these, except in older code where
// they were mandatory. Nonetheless, we have to ensure that every such
// predicate is satisfied, so they form a kind of base set of requirements
// for the type.
let mut explicit_outlives_predicates = map();
for def_id in all_types() {
    let explicit_predicates = tcx.explicit_predicates(def_id);
    let filtered_predicates = explicit_predicates.iter().filter_map(is_outlives_predicate).collect();
    explicit_outlives_predicates.insert(
        def_id,
        filtered_predicates);
}

// Create the sets of inferred predicates for each type. These sets
// are initially empty but will grow during the inference step.
let mut inferred_outlives_predicates = map();
for def_id in all_types() {
    inferred_outlives_predicates.insert(def_id, empty_set());
}

// Now comes the inference part. We iterate over each type and figure out,
// for each of its fields, what outlives predicates are needed to make that field's
// type well-formed. Those get added to the `inferred_outlives_predicates` set
// for that type.
let mut changed = true;
while changed {
    changed = false;
    for def_id in all_types() {
        for field_ty in the HIR type definition {
            let required_predicates = required_predicates_for_type_to_be_wf(field_ty);
            inferred_outlives_predicates.extend(required_predicates);
            if new predicates were added { changed = true; }
        }
    }
}

// When we are done, `inferred_outlives_predicates` is the resulting map.
