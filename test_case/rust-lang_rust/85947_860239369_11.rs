
                    span: lib.rs:1:23: 1:32 (#0),
                    bound_generic_params: [],
                    bounded_ty: Ty {
                        hir_id: HirId {
                            owner: DefId(0:3 ~ lib[317d]::A),
                            local_id: 10,
                        },
                        kind: Path(
                            Resolved(
                                None,
                                Path {
                                    span: lib.rs:1:23: 1:24 (#0),
                                    res: Def(
                                        TyParam,
                                        DefId(0:5 ~ lib[317d]::A::T),
                                    ),
                                    segments: [
                                        PathSegment {
                                            ident: T#0,
                                            hir_id: Some(
                                                HirId {
                                                    owner: DefId(0:3 ~ lib[317d]::A),
                                                    local_id: 11,
                                                },
                                            ),
                                            res: Some(
                                                Def(
                                                    TyParam,
                                                    DefId(0:5 ~ lib[317d]::A::T),
                                                ),
                                            ),
                                            args: None,
                                            infer_args: false,
                                        },
                                    ],
                                },
                            ),
                        ),
                        span: lib.rs:1:23: 1:24 (#0),
                    },
