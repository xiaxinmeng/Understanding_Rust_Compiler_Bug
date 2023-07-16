
DefId(0:3 ~ test[8787]::is_awesome):
Expr {
    ty: bool,
    temp_lifetime: Some(
        Node(17),
    ),
    span: test.rs:1:39: 3:2 (#0),
    kind: Scope {
        region_scope: Destruction(17),
        lint_level: Inherited,
        value: Expr {
            ty: bool,
            temp_lifetime: Some(
                Node(17),
            ),
            span: test.rs:1:39: 3:2 (#0),
            kind: Scope {
                region_scope: Node(17),
                lint_level: Explicit(
                    HirId {
                        owner: DefId(0:3 ~ test[8787]::is_awesome),
                        local_id: 17,
                    },
                ),
                value: Expr {
                    ty: bool,
                    temp_lifetime: Some(
                        Node(17),
                    ),
                    span: test.rs:1:39: 3:2 (#0),
                    kind: Block {
                        body: Block {
                            targeted_by_break: false,
                            region_scope: Node(16),
                            opt_destruction_scope: None,
                            span: test.rs:1:39: 3:2 (#0),
                            stmts: [],
                            expr: Some(
                                Expr {
                                    ty: bool,
                                    temp_lifetime: Some(
                                        Node(17),
                                    ),
                                    span: /Users/tous/Documents/Dev/rust/library/core/src/macros/mod.rs:319:9: 322:10 (#6),
                                    kind: Scope {
                                        region_scope: Node(15),
                                        lint_level: Explicit(
                                            HirId {
                                                owner: DefId(0:3 ~ test[8787]::is_awesome),
                                                local_id: 15,
                                            },
                                        ),
                                        value: Expr {
                                            ty: bool,
                                            temp_lifetime: Some(
                                                Node(17),
                                            ),
                                            span: /Users/tous/Documents/Dev/rust/library/core/src/macros/mod.rs:319:9: 322:10 (#6),
                                            kind: Match {
                                                scrutinee: Expr {
                                                    ty: &str,
                                                    temp_lifetime: Some(
                                                        Node(17),
                                                    ),
                                                    span: test.rs:2:14: 2:18 (#0),
                                                    kind: Scope {
                                                        region_scope: Node(4),
                                                        lint_level: Explicit(
                                                            HirId {
                                                                owner: DefId(0:3 ~ test[8787]::is_awesome),
                                                                local_id: 4,
                                                            },
                                                        ),
                                                        value: Expr {
                                                            ty: &str,
                                                            temp_lifetime: Some(
                                                                Node(17),
                                                            ),
                                                            span: test.rs:2:14: 2:18 (#0),
                                                            kind: VarRef {
                                                                id: HirId {
                                                                    owner: DefId(0:3 ~ test[8787]::is_awesome),
                                                                    local_id: 2,
                                                                },
                                                            },
                                                        },
                                                    },
                                                },
                                                arms: [
                                                    Arm {
                                                        pattern: Pat {
                                                            ty: &str,
                                                            span: /Users/tous/Documents/Dev/rust/library/core/src/macros/mod.rs:320:16: 320:24 (#6),
                                                            kind: Or {
                                                                pats: [
                                                                    Pat {
                                                                        ty: &str,
                                                                        span: test.rs:2:20: 2:26 (#0),
                                                                        kind: Constant {
                                                                            value: Const {
                                                                                ty: &str,
                                                                                val: Value(
                                                                                    Slice {
                                                                                        data: Allocation {
                                                                                            bytes: [
                                                                                                82,
                                                                                                117,
                                                                                                115,
                                                                                                116,
                                                                                            ],
                                                                                            relocations: Relocations(
                                                                                                SortedMap {
                                                                                                    data: [],
                                                                                                },
                                                                                            ),
                                                                                            init_mask: InitMask {
                                                                                                blocks: [
                                                                                                    15,
                                                                                                ],
                                                                                                len: Size {
                                                                                                    raw: 4,
                                                                                                },
                                                                                            },
                                                                                            size: Size {
                                                                                                raw: 4,
                                                                                            },
                                                                                            align: Align {
                                                                                                pow2: 0,
                                                                                            },
                                                                                            mutability: Not,
                                                                                            extra: (),
                                                                                        },
                                                                                        start: 0,
                                                                                        end: 4,
                                                                                    },
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                    Pat {
                                                                        ty: &str,
                                                                        span: test.rs:2:29: 2:37 (#0),
                                                                        kind: Constant {
                                                                            value: Const {
                                                                                ty: &str,
                                                                                val: Value(
                                                                                    Slice {
                                                                                        data: Allocation {
                                                                                            bytes: [
                                                                                                70,
                                                                                                101,
                                                                                                114,
                                                                                                114,
                                                                                                105,
                                                                                                115,
                                                                                            ],
                                                                                            relocations: Relocations(
                                                                                                SortedMap {
                                                                                                    data: [],
                                                                                                },
                                                                                            ),
                                                                                            init_mask: InitMask {
                                                                                                blocks: [
                                                                                                    63,
                                                                                                ],
                                                                                                len: Size {
                                                                                                    raw: 6,
                                                                                                },
                                                                                            },
                                                                                            size: Size {
                                                                                                raw: 6,
                                                                                            },
                                                                                            align: Align {
                                                                                                pow2: 0,
                                                                                            },
                                                                                            mutability: Not,
                                                                                            extra: (),
                                                                                        },
                                                                                        start: 0,
                                                                                        end: 6,
                                                                                    },
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        },
                                                        guard: None,
                                                        body: Expr {
                                                            ty: bool,
                                                            temp_lifetime: Some(
                                                                Node(11),
                                                            ),
                                                            span: /Users/tous/Documents/Dev/rust/library/core/src/macros/mod.rs:320:48: 320:52 (#6),
                                                            kind: Scope {
                                                                region_scope: Destruction(11),
                                                                lint_level: Inherited,
                                                                value: Expr {
                                                                    ty: bool,
                                                                    temp_lifetime: Some(
                                                                        Node(11),
                                                                    ),
                                                                    span: /Users/tous/Documents/Dev/rust/library/core/src/macros/mod.rs:320:48: 320:52 (#6),
                                                                    kind: Scope {
                                                                        region_scope: Node(11),
                                                                        lint_level: Explicit(
                                                                            HirId {
                                                                                owner: DefId(0:3 ~ test[8787]::is_awesome),
                                                                                local_id: 11,
                                                                            },
                                                                        ),
                                                                        value: Expr {
                                                                            ty: bool,
                                                                            temp_lifetime: Some(
                                                                                Node(11),
                                                                            ),
                                                                            span: /Users/tous/Documents/Dev/rust/library/core/src/macros/mod.rs:320:48: 320:52 (#6),
                                                                            kind: Literal {
                                                                                literal: Const {
                                                                                    ty: bool,
                                                                                    val: Value(
                                                                                        Scalar(
                                                                                            0x01,
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                                user_ty: None,
                                                                                const_id: None,
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        lint_level: Explicit(
                                                            HirId {
                                                                owner: DefId(0:3 ~ test[8787]::is_awesome),
                                                                local_id: 10,
                                                            },
                                                        ),
                                                        scope: Node(10),
                                                        span: /Users/tous/Documents/Dev/rust/library/core/src/macros/mod.rs:320:16: 320:52 (#6),
                                                    },
                                                    Arm {
                                                        pattern: Pat {
                                                            ty: &str,
                                                            span: /Users/tous/Documents/Dev/rust/library/core/src/macros/mod.rs:321:13: 321:14 (#6),
                                                            kind: Wild,
                                                        },
                                                        guard: None,
                                                        body: Expr {
                                                            ty: bool,
                                                            temp_lifetime: Some(
                                                                Node(14),
                                                            ),
                                                            span: /Users/tous/Documents/Dev/rust/library/core/src/macros/mod.rs:321:18: 321:23 (#6),
                                                            kind: Scope {
                                                                region_scope: Destruction(14),
                                                                lint_level: Inherited,
                                                                value: Expr {
                                                                    ty: bool,
                                                                    temp_lifetime: Some(
                                                                        Node(14),
                                                                    ),
                                                                    span: /Users/tous/Documents/Dev/rust/library/core/src/macros/mod.rs:321:18: 321:23 (#6),
                                                                    kind: Scope {
                                                                        region_scope: Node(14),
                                                                        lint_level: Explicit(
                                                                            HirId {
                                                                                owner: DefId(0:3 ~ test[8787]::is_awesome),
                                                                                local_id: 14,
                                                                            },
                                                                        ),
                                                                        value: Expr {
                                                                            ty: bool,
                                                                            temp_lifetime: Some(
                                                                                Node(14),
                                                                            ),
                                                                            span: /Users/tous/Documents/Dev/rust/library/core/src/macros/mod.rs:321:18: 321:23 (#6),
                                                                            kind: Literal {
                                                                                literal: Const {
                                                                                    ty: bool,
                                                                                    val: Value(
                                                                                        Scalar(
                                                                                            0x00,
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                                user_ty: None,
                                                                                const_id: None,
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        lint_level: Explicit(
                                                            HirId {
                                                                owner: DefId(0:3 ~ test[8787]::is_awesome),
                                                                local_id: 13,
                                                            },
                                                        ),
                                                        scope: Node(13),
                                                        span: /Users/tous/Documents/Dev/rust/library/core/src/macros/mod.rs:321:13: 321:23 (#6),
                                                    },
                                                ],
                                            },
                                        },
                                    },
                                },
                            ),
                            safety_mode: Safe,
                        },
                    },
                },
            },
        },
    },
}


