
DEBUG rustc_traits::evaluate_obligation evaluate_obligation(Canonical {
    max_universe: U0,
    variables: [
        CanonicalVarInfo {
            kind: Region(
                U0,
            ),
        },
    ],
    value: ParamEnvAnd {
        param_env: ParamEnv {
            caller_bounds: [],
            reveal: All,
        },
        value: TraitPredicate(<sc_service::Arc<dyn sp_state_machine::trie_backend_essence::Storage<sp_runtime::traits::BlakeTwo256>> as sp_state_machine::trie_backend_essence::TrieBackendStorage<sp_runtime::traits::BlakeTwo256>>),
    },
}) = Ok(EvaluatedToOk)
