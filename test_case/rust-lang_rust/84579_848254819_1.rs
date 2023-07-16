
[src/librustdoc/clean/mod.rs:1105] &generics = Generics {
    params: [],
    where_predicates: [
        BoundPredicate {
            ty: QPath {
                name: "IntoIter",
                self_type: Generic(
                    "Self",
                ),
                self_def_id: None,
                trait_: ResolvedPath {
                    path: Path {
                        global: false,
                        res: Err,
                        segments: [
                            PathSegment {
                                name: "IntoIterator",
                                args: AngleBracketed {
                                    args: [],
                                    bindings: [],
                                },
                            },
                        ],
                    },
                    param_names: None,
                    did: DefId(19:3 ~ b[78df]::IntoIterator),
                    is_generic: false,
                },
            },
            bounds: [
                TraitBound(
                    PolyTrait {
                        trait_: ResolvedPath {
                            path: Path {
                                global: false,
                                res: Err,
                                segments: [
                                    PathSegment {
                                        name: "Sized",
                                        args: AngleBracketed {
                                            args: [],
                                            bindings: [],
                                        },
                                    },
                                ],
                            },
                            param_names: None,
                            did: DefId(2:2839 ~ core[51e2]::marker::Sized),
                            is_generic: false,
                        },
                        generic_params: [],
                    },
                    None,
                ),
            ],
        },
        BoundPredicate {
            ty: QPath {
                name: "IntoIter",
                self_type: Generic(
                    "Self",
                ),
                self_def_id: None,
                trait_: ResolvedPath {
                    path: Path {
                        global: false,
                        res: Err,
                        segments: [
                            PathSegment {
                                name: "IntoIterator",
                                args: AngleBracketed {
                                    args: [],
                                    bindings: [],
                                },
                            },
                        ],
                    },
                    param_names: None,
                    did: DefId(19:3 ~ b[78df]::IntoIterator),
                    is_generic: false,
                },
            },
            bounds: [
                TraitBound(
                    PolyTrait {
                        trait_: ResolvedPath {
                            path: Path {
                                global: false,
                                res: Err,
                                segments: [
                                    PathSegment {
                                        name: "Iterator",
                                        args: AngleBracketed {
                                            args: [],
                                            bindings: [],
                                        },
                                    },
                                ],
                            },
                            param_names: None,
                            did: DefId(2:7246 ~ core[51e2]::iter::traits::iterator::Iterator),
                            is_generic: false,
                        },
                        generic_params: [],
                    },
                    None,
                ),
            ],
        },
        EqPredicate {
            lhs: QPath {
                name: "Item",
                self_type: QPath {
                    name: "IntoIter",
                    self_type: Generic(
                        "Self",
                    ),
                    self_def_id: None,
                    trait_: ResolvedPath {
                        path: Path {
                            global: false,
                            res: Err,
                            segments: [
                                PathSegment {
                                    name: "IntoIterator",
                                    args: AngleBracketed {
                                        args: [],
                                        bindings: [],
                                    },
                                },
                            ],
                        },
                        param_names: None,
                        did: DefId(19:3 ~ b[78df]::IntoIterator),
                        is_generic: false,
                    },
                },
                self_def_id: None,
                trait_: ResolvedPath {
                    path: Path {
                        global: false,
                        res: Err,
                        segments: [
                            PathSegment {
                                name: "Iterator",
                                args: AngleBracketed {
                                    args: [],
                                    bindings: [],
                                },
                            },
                        ],
                    },
                    param_names: None,
                    did: DefId(2:7246 ~ core[51e2]::iter::traits::iterator::Iterator),
                    is_generic: false,
                },
            },
            rhs: QPath {
                name: "Item",
                self_type: Generic(
                    "Self",
                ),
                self_def_id: None,
                trait_: ResolvedPath {
                    path: Path {
                        global: false,
                        res: Err,
                        segments: [
                            PathSegment {
                                name: "IntoIterator",
                                args: AngleBracketed {
                                    args: [],
                                    bindings: [],
                                },
                            },
                        ],
                    },
                    param_names: None,
                    did: DefId(19:3 ~ b[78df]::IntoIterator),
                    is_generic: false,
                },
            },
        },
    ],
}
