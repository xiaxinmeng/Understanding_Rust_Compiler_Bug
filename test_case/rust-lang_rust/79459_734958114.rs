
query stack during panic:
#0 [normalize_projection_ty] normalizing `Canonical { max_universe: U0, variables: [], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [TraitPredicate(<Selection as std::marker::Sized>), TraitPredicate(<Source as std::marker::Sized>)], reveal: UserFacing }, value: ProjectionTy { substs: [Source, Selection], item_def_id: DefId(0:11 ~ normalize_type_length_limit[8787]::SelectDsl::Output) } } }`
end of query stack
