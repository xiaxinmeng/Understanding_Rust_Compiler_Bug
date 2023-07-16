text
Canonical {
    max_universe: U0,
    variables: [
        CanonicalVarInfo { kind: Region(U0) },
        CanonicalVarInfo { kind: Region(U0) }
    ],
    value: ParamEnvAnd {
        param_env: ParamEnv {
            caller_bounds: [
                Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion { var: 0, kind: BrAnon(0) }), ReLateBound(DebruijnIndex(1), BoundRegion { var: 1, kind: BrAnon(1) })), []),
                Binder(OutlivesPredicate(scenarios::delete::TwoDeleteMultiExprsFromRubOneMeasurementOneChunk, ReLateBound(DebruijnIndex(1), BoundRegion { var: 1, kind: BrAnon(1) })), [])
            ],
        reveal: UserFacing
        },
        value: Binder(TraitPredicate(<std::sync::Arc<server::Db> as std::marker::Sync>), [])
    }
}
