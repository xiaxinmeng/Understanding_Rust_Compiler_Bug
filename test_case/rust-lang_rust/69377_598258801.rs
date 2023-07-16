
Crate {
    ...
    items: {
        ...
        HirId {
            owner: DefIndex(32),
            local_id: 0,
        }: Item {
            ident: impl_trait_in_bindings#0,
            hir_id: HirId {
                owner: DefIndex(32),
                local_id: 0,
            },
            attrs: [],
            kind: Mod(
                Mod {
                    inner: $DIR/bound-normalization-pass.rs:62:5: 77:2,
                    item_ids: [
                        ...
                        ItemId {
                            id: HirId {
                                owner: DefIndex(39),
                                local_id: 0,
                            },
                        },
                        ItemId {
                            id: HirId {
                                owner: DefIndex(41),
                                local_id: 0,
                            },
                        },
                    ],
                },
            ),
            vis: Spanned {
                node: Inherited,
                span: $DIR/bound-normalization-pass.rs:61:1: 61:1,
            },
            span: $DIR/bound-normalization-pass.rs:61:1: 77:2,
        },
        ...
        HirId {
            owner: DefIndex(41),
            local_id: 0,
        }: Item {
            ident: foo#0,
            hir_id: HirId {
                owner: DefIndex(41),
                local_id: 0,
            },
            attrs: [],
            kind: Fn(
                FnSig {
                    header: FnHeader {
                        unsafety: Normal,
                        constness: NotConst,
                        asyncness: NotAsync,
                        abi: Rust,
                    },
                    decl: FnDecl {
                        inputs: [],
                        output: DefaultReturn(
                            $DIR/bound-normalization-pass.rs:74:35: 74:35,
                        ),
                        c_variadic: false,
                        implicit_self: None,
                    },
                },
                Generics {
                    params: [
                        GenericParam {
                            hir_id: HirId {
                                owner: DefIndex(41),
                                local_id: 15,
                            },
                            name: Plain(
                                T#0,
                            ),
                            attrs: [],
                            bounds: [
                                Trait(
                                    PolyTraitRef {
                                        bound_generic_params: [],
                                        trait_ref: TraitRef {
                                            path: path(Trait<Assoc = u32>),
                                            hir_ref_id: HirId {
                                                owner: DefIndex(41),
                                                local_id: 14,
                                            },
                                        },
                                        span: $DIR/bound-normalization-pass.rs:74:15: 74:31,
                                    },
                                    None,
                                ),
                            ],
                            span: $DIR/bound-normalization-pass.rs:74:12: 74:13,
                            pure_wrt_drop: false,
                            kind: Type {
                                default: None,
                                synthetic: None,
                            },
                        },
                    ],
                    where_clause: WhereClause {
                        predicates: [],
                        span: $DIR/bound-normalization-pass.rs:74:34: 74:34,
                    },
                    span: $DIR/bound-normalization-pass.rs:74:11: 74:32,
                },
                BodyId {
                    hir_id: HirId {
                        owner: DefIndex(41),
                        local_id: 9,
                    },
                },
            ),
            vis: Spanned {
                node: Inherited,
                span: $DIR/bound-normalization-pass.rs:74:5: 74:5,
            },
            span: $DIR/bound-normalization-pass.rs:74:5: 76:6,
        },
        HirId {
            owner: DefIndex(43),
            local_id: 0,
        }: Item {
            ident: #0,
            hir_id: HirId {
                owner: DefIndex(43),
                local_id: 0,
            },
            attrs: [],
            kind: OpaqueTy(
                OpaqueTy {
                    generics: Generics {
                        params: [],
                        where_clause: WhereClause {
                            predicates: [],
                            span: $DIR/bound-normalization-pass.rs:75:16: 75:45,
                        },
                        span: $DIR/bound-normalization-pass.rs:75:16: 75:45,
                    },
                    bounds: [
                        Trait(
                            PolyTraitRef {
                                bound_generic_params: [],
                                trait_ref: TraitRef {
                                    path: path(FooLike<Output = T::Assoc>),
                                    hir_ref_id: HirId {
                                        owner: DefIndex(43),
                                        local_id: 7,
                                    },
                                },
                                span: $DIR/bound-normalization-pass.rs:75:21: 75:45,
                            },
                            None,
                        ),
                    ],
                    impl_trait_fn: Some(
                        DefId(0:41 ~ bound_normalization_pass[317d]::impl_trait_in_bindings[0]::foo[0]),
                    ),
                    origin: Misc,
                },
            ),
            vis: Spanned {
                node: Inherited,
                span: $DIR/bound-normalization-pass.rs:75:16: 75:16,
            },
            span: $DIR/bound-normalization-pass.rs:75:16: 75:45,
        },
        ...
    },
    trait_items: {
        ...
        TraitItemId {
            hir_id: HirId {
                owner: DefIndex(36),
                local_id: 0,
            },
        }: TraitItem {
            ident: Output#0,
            hir_id: HirId {
                owner: DefIndex(36),
                local_id: 0,
            },
            attrs: [],
            generics: Generics {
                params: [],
                where_clause: WhereClause {
                    predicates: [],
                    span: $DIR/bound-normalization-pass.rs:64:32: 64:32,
                },
                span: $DIR/bound-normalization-pass.rs:64:32: 64:32,
            },
            kind: Type(
                [],
                None,
            ),
            span: $DIR/bound-normalization-pass.rs:64:21: 64:33,
        },
        TraitItemId {
            hir_id: HirId {
                owner: DefIndex(40),
                local_id: 0,
            },
        }: TraitItem {
            ident: Assoc#0,
            hir_id: HirId {
                owner: DefIndex(40),
                local_id: 0,
            },
            attrs: [],
            generics: Generics {
                params: [],
                where_clause: WhereClause {
                    predicates: [],
                    span: $DIR/bound-normalization-pass.rs:71:19: 71:19,
                },
                span: $DIR/bound-normalization-pass.rs:71:19: 71:19,
            },
            kind: Type(
                [],
                None,
            ),
            span: $DIR/bound-normalization-pass.rs:71:9: 71:20,
        },
        ...
    },
    impl_items: {
        ...
        ImplItemId {
            hir_id: HirId {
                owner: DefIndex(38),
                local_id: 0,
            },
        }: ImplItem {
            ident: Output#0,
            hir_id: HirId {
                owner: DefIndex(38),
                local_id: 0,
            },
            vis: Spanned {
                node: Inherited,
                span: $DIR/bound-normalization-pass.rs:67:9: 67:9,
            },
            defaultness: Final,
            attrs: [],
            generics: Generics {
                params: [],
                where_clause: WhereClause {
                    predicates: [],
                    span: $DIR/bound-normalization-pass.rs:67:20: 67:20,
                },
                span: $DIR/bound-normalization-pass.rs:67:20: 67:20,
            },
            kind: TyAlias(
                type(u32),
            ),
            span: $DIR/bound-normalization-pass.rs:67:9: 67:27,
        },
        ...
    },
    bodies: {
        BodyId {
            hir_id: HirId {
                owner: DefIndex(41),
                local_id: 9,
            },
        }: Body {
            params: [],
            value: expr(HirId { owner: DefIndex(41), local_id: 9 }: { let _:  = Foo; }),
            generator_kind: None,
        },
    },
    trait_impls: {
        ...
        DefId(0:45 ~ bound_normalization_pass[317d]::opaque_types[0]::Implemented[0]): [
            HirId {
                owner: DefIndex(47),
                local_id: 0,
            },
        ],
        ...
    },
    body_ids: [
        ...
        BodyId {
            hir_id: HirId {
                owner: DefIndex(41),
                local_id: 9,
            },
        },
        ...
    ],
    modules: {
        ...
        HirId {
            owner: DefIndex(32),
            local_id: 0,
        }: ModuleItems {
            items: {
                ...
                HirId {
                    owner: DefIndex(41),
                    local_id: 0,
                },
                HirId {
                    owner: DefIndex(43),
                    local_id: 0,
                },
            },
            trait_items: {
                TraitItemId {
                    hir_id: HirId {
                        owner: DefIndex(36),
                        local_id: 0,
                    },
                },
                TraitItemId {
                    hir_id: HirId {
                        owner: DefIndex(40),
                        local_id: 0,
                    },
                },
            },
            ...
        },
        ...
    },
    proc_macros: [],
}
