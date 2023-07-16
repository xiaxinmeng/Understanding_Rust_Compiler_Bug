
DEBUG rustc_trait_selection::traits::error_reporting::suggestions suggest_await_before_try: obligation=Obligation(predicate=Binder(TraitPredicate(<impl std::future::Future as std::ops::Try>), []), depth=0), span=issue-84841.rs:3:5: 3:12 (#3), trait_ref=Binder(<impl std::future::Future as std::ops::Try>, []), trait_ref_self_ty=Binder(impl std::future::Future, [])
DEBUG rustc_trait_selection::traits type_implements_trait: trait_def_id=DefId(2:12109 ~ core[ef11]::future::future::Future), type=impl std::future::Future, params=[], param_env=ParamEnv { caller_bounds: [], reveal: UserFacing }
thread 'rustc' panicked at 'index out of bounds: the len is 0 but the index is 0', /usr/local/google/home/tmandry/.cargo/registry/src/github.com-1ecc6299db9ec823/ena-0.14.0/src/snapshot_vec.rs:199:10
