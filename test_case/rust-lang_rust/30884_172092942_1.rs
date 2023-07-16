
thread 'rustc' panicked at 'adding a def'n for node-id 200897 and data Binding(_result(53163)) but a previous def'n exists:
DefData {
    key: DefKey {
        parent: Some(DefIndex(1701)),
        disambiguated_data: DisambiguatedDefPathData {
            data: Binding(_result(53163)),
            disambiguator: 1
        }
    },
    node_id: 200897
}', src/librustc/front/map/definitions.rs:146
