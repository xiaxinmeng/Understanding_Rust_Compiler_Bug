
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::core::fmt::Debug for Declar<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Declar::Axiom {
                    name: ref __self_0,
                    uparams: ref __self_1,
                    type_: ref __self_2,
                    is_unsafe: ref __self_3,
                },) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Axiom");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "name", &&(*__self_0));
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "uparams",
                        &&(*__self_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "type_",
                        &&(*__self_2),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "is_unsafe",
                        &&(*__self_3),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
                (&Declar::Definition {
                    name: ref __self_0,
                    uparams: ref __self_1,
                    type_: ref __self_2,
                    val: ref __self_3,
                    hint: ref __self_4,
                    is_unsafe: ref __self_5,
                },) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Definition");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "name", &&(*__self_0));
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "uparams",
                        &&(*__self_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "type_",
                        &&(*__self_2),
                    );
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "val", &&(*__self_3));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "hint", &&(*__self_4));
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "is_unsafe",
                        &&(*__self_5),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
                (&Declar::Theorem {
                    name: ref __self_0,
                    uparams: ref __self_1,
                    type_: ref __self_2,
                    val: ref __self_3,
                },) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Theorem");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "name", &&(*__self_0));
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "uparams",
                        &&(*__self_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "type_",
                        &&(*__self_2),
                    );
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "val", &&(*__self_3));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
                (&Declar::Opaque {
                    name: ref __self_0,
                    uparams: ref __self_1,
                    type_: ref __self_2,
                    val: ref __self_3,
                },) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Opaque");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "name", &&(*__self_0));
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "uparams",
                        &&(*__self_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "type_",
                        &&(*__self_2),
                    );
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "val", &&(*__self_3));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
                (&Declar::Quot {
                    name: ref __self_0,
                    uparams: ref __self_1,
                    type_: ref __self_2,
                },) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Quot");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "name", &&(*__self_0));
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "uparams",
                        &&(*__self_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "type_",
                        &&(*__self_2),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
                (&Declar::Inductive {
                    name: ref __self_0,
                    uparams: ref __self_1,
                    type_: ref __self_2,
                    num_params: ref __self_3,
                    all_ind_names: ref __self_4,
                    all_cnstr_names: ref __self_5,
                    is_unsafe: ref __self_6,
                },) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Inductive");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "name", &&(*__self_0));
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "uparams",
                        &&(*__self_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "type_",
                        &&(*__self_2),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "num_params",
                        &&(*__self_3),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "all_ind_names",
                        &&(*__self_4),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "all_cnstr_names",
                        &&(*__self_5),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "is_unsafe",
                        &&(*__self_6),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
                (&Declar::Constructor {
                    name: ref __self_0,
                    uparams: ref __self_1,
                    type_: ref __self_2,
                    parent_name: ref __self_3,
                    num_fields: ref __self_4,
                    num_params: ref __self_5,
                    is_unsafe: ref __self_6,
                },) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Constructor");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "name", &&(*__self_0));
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "uparams",
                        &&(*__self_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "type_",
                        &&(*__self_2),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "parent_name",
                        &&(*__self_3),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "num_fields",
                        &&(*__self_4),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "num_params",
                        &&(*__self_5),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "is_unsafe",
                        &&(*__self_6),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
                (&Declar::Recursor {
                    name: ref __self_0,
                    uparams: ref __self_1,
                    type_: ref __self_2,
                    all_names: ref __self_3,
                    num_params: ref __self_4,
                    num_indices: ref __self_5,
                    num_motives: ref __self_6,
                    num_minors: ref __self_7,
                    major_idx: ref __self_8,
                    rec_rules: ref __self_9,
                    is_k: ref __self_10,
                    is_unsafe: ref __self_11,
                },) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Recursor");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "name", &&(*__self_0));
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "uparams",
                        &&(*__self_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "type_",
                        &&(*__self_2),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "all_names",
                        &&(*__self_3),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "num_params",
                        &&(*__self_4),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "num_indices",
                        &&(*__self_5),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "num_motives",
                        &&(*__self_6),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "num_minors",
                        &&(*__self_7),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "major_idx",
                        &&(*__self_8),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "rec_rules",
                        &&(*__self_9),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "is_k",
                        &&(*__self_10),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "is_unsafe",
                        &&(*__self_11),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
