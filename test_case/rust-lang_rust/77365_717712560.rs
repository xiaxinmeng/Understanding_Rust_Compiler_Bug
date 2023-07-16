rust
2:rustcDEBUG rustc_typeck::check::writeback writeback: typeck results for DefId(0:3 ~ issue_77365[317d]::main) are TypeckResults {
2:rustc    hir_owner: DefId(0:3 ~ issue_77365[317d]::main),
2:rustc    type_dependent_defs: {
2:rustc        28: Ok(
2:rustc            (
2:rustc                AssocFn,
2:rustc                DefId(5:4646 ~ alloc[fd51]::vec::{impl#0}::retain),
2:rustc            ),
2:rustc        ),
2:rustc        13: Ok(
2:rustc            (
2:rustc                AssocFn,
2:rustc                DefId(5:4622 ~ alloc[fd51]::vec::{impl#0}::new),
2:rustc            ),
2:rustc        ),
2:rustc    },
2:rustc    field_indices: {},
2:rustc    node_types: {
2:rustc        20: [closure@src/test/ui/issues/issue-77365.rs:3:13: 3:21],
2:rustc        17: &i32,
2:rustc        14: std::vec::Vec<i32>,
2:rustc        11: std::vec::Vec<i32>,
2:rustc        5: i32,
2:rustc        2: [closure@src/test/ui/issues/issue-77365.rs:3:13: 3:21],
2:rustc        31: (),
2:rustc        28: (),
2:rustc        25: std::vec::Vec<i32>,
2:rustc        19: bool,
2:rustc        13: fn() -> std::vec::Vec<i32> {std::vec::Vec::<i32>::new},
2:rustc        4: std::vec::Vec<i32>,
2:rustc        1: std::vec::Vec<i32>,
2:rustc        30: (),
2:rustc        27: [closure@src/test/ui/issues/issue-77365.rs:3:13: 3:21],
2:rustc        21: [closure@src/test/ui/issues/issue-77365.rs:3:13: 3:21],
2:rustc        18: &i32,
2:rustc        15: std::vec::Vec<i32>,
2:rustc        3: &i32,
2:rustc    },
2:rustc    node_substs: {
2:rustc        28: [
2:rustc            i32,
2:rustc            [closure@src/test/ui/issues/issue-77365.rs:3:13: 3:21],
2:rustc        ],
2:rustc        13: [
2:rustc            i32,
2:rustc        ],
2:rustc    },
2:rustc    user_provided_types: {
2:rustc        4: Canonical {
2:rustc            max_universe: U0,
2:rustc            variables: [],
2:rustc            value: Ty(
2:rustc                std::vec::Vec<i32>,
2:rustc            ),
2:rustc        },
2:rustc        13: Canonical {
2:rustc            max_universe: U0,
2:rustc            variables: [
2:rustc                CanonicalVarInfo {
2:rustc                    kind: Ty(
2:rustc                        General(
2:rustc                            U0,
2:rustc                        ),
2:rustc                    ),
2:rustc                },
2:rustc                CanonicalVarInfo {
2:rustc                    kind: Ty(
2:rustc                        General(
2:rustc                            U0,
2:rustc                        ),
2:rustc                    ),
2:rustc                },
2:rustc            ],
2:rustc            value: TypeOf(
2:rustc                DefId(5:4622 ~ alloc[fd51]::vec::{impl#0}::new),
2:rustc                UserSubsts {
2:rustc                    substs: [
2:rustc                        ^0,
2:rustc                    ],
2:rustc                    user_self_ty: Some(
2:rustc                        UserSelfTy {
2:rustc                            impl_def_id: DefId(5:4620 ~ alloc[fd51]::vec::{impl#0}),
2:rustc                            self_ty: std::vec::Vec<^1>,
2:rustc                        },
2:rustc                    ),
2:rustc                },
2:rustc            ),
2:rustc        },
2:rustc    },
2:rustc    user_provided_sigs: {
2:rustc        DefId(0:4 ~ issue_77365[317d]::main::{closure#0}): Canonical {
2:rustc            max_universe: U0,
2:rustc            variables: [
2:rustc                CanonicalVarInfo {
2:rustc                    kind: Ty(
2:rustc                        General(
2:rustc                            U0,
2:rustc                        ),
2:rustc                    ),
2:rustc                },
2:rustc                CanonicalVarInfo {
2:rustc                    kind: Ty(
2:rustc                        General(
2:rustc                            U0,
2:rustc                        ),
2:rustc                    ),
2:rustc                },
2:rustc            ],
2:rustc            value: Binder(
2:rustc                ([^1_0]; c_variadic: false)->^1_1,
2:rustc            ),
2:rustc        },
2:rustc    },
2:rustc    adjustments: {
2:rustc        25: [
2:rustc            Borrow(Ref(ReErased, Mut { allow_two_phase_borrow: Yes })) -> &mut Vec<i32>,
2:rustc        ],
2:rustc    },
2:rustc    pat_binding_modes: {
2:rustc        1: BindByValue(
2:rustc            Mut,
2:rustc        ),
2:rustc        2: BindByValue(
2:rustc            Not,
2:rustc        ),
2:rustc    },
2:rustc    pat_adjustments: {},
2:rustc    upvar_capture_map: {},
2:rustc    closure_kind_origins: {},
2:rustc    liberated_fn_sigs: {
2:rustc        0: ([]; c_variadic: false)->(),
2:rustc        20: ([&i32]; c_variadic: false)->bool,
2:rustc    },
2:rustc    fru_field_types: {},
2:rustc    coercion_casts: {},
2:rustc    used_trait_imports: {},
2:rustc    tainted_by_errors: Some(
2:rustc        ErrorReported,
2:rustc    ),
2:rustc    concrete_opaque_types: {},
2:rustc    closure_captures: {},
2:rustc    generator_interior_types: [],
2:rustc}
