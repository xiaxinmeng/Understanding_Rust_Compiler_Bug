
kind: Call(
    Expr {
        hir_id: HirId {
            owner: OwnerId {
                def_id: DefId(0:6 ~ sample[997c]::{impl#0}::test),
            },
            local_id: 4,
        },
        kind: Path(
            TypeRelative(
                Ty {
                    hir_id: HirId {
                        owner: OwnerId {
                            def_id: DefId(0:6 ~ sample[997c]::{impl#0}::test),
                        },
                        local_id: 2,
                    },
                    kind: Path(
                        Resolved(
                            None,
                            Path {
                                span: sample.rs:8:9: 8:12 (#0),
                                res: Def(
                                    Struct,
                                    DefId(0:3 ~ sample[997c]::Foo),
                                ),
                                segments: [
                                    PathSegment {
                                        ident: Foo#0,
                                        hir_id: HirId {
                                            owner: OwnerId {
                                                def_id: DefId(0:6 ~ sample[997c]::{impl#0}::test),
                                            },
                                            local_id: 1,
                                        },
                                        res: Def(
                                            Struct,
                                            DefId(0:3 ~ sample[997c]::Foo),
                                        ),
                                        args: None,
                                        infer_args: true,
                                    },
                                ],
                            },
                        ),
                    ),
                    # Fixed
                    span: sample.rs:8:9: 8:12 (#0),
                },
                PathSegment {
                    ident: new#0,
                    hir_id: HirId {
                        owner: OwnerId {
                            def_id: DefId(0:6 ~ sample[997c]::{impl#0}::test),
                        },
                        local_id: 3,
                    },
                    res: Err,
                    args: None,
                    infer_args: true,
                },
            ),
        ),
        span: sample.rs:8:9: 8:17 (#0),
    },
    [],
),
