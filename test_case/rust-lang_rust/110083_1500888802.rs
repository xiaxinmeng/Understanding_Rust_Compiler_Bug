plain

---- [ui] tests/ui/thir-print/thir-tree-match.rs stdout ----
diff of stdout:

105                                                                                                                 did: DefId(0:10 ~ thir_tree_match[fcf8]::Foo)
106                                                                                                                 variants: [VariantDef { def_id: DefId(0:11 ~ thir_tree_match[fcf8]::Foo::FooOne), ctor: Some((Fn, DefId(0:12 ~ thir_tree_match[fcf8]::Foo::FooOne::{constructor#0}))), name: "FooOne", discr: Relative(0), fields: [FieldDef { did: DefId(0:13 ~ thir_tree_match[fcf8]::Foo::FooOne::0), name: "0", vis: Restricted(DefId(0:0 ~ thir_tree_match[fcf8])) }], flags: NO_VARIANT_FLAGS }, VariantDef { def_id: DefId(0:14 ~ thir_tree_match[fcf8]::Foo::FooTwo), ctor: Some((Const, DefId(0:15 ~ thir_tree_match[fcf8]::Foo::FooTwo::{constructor#0}))), name: "FooTwo", discr: Relative(1), fields: [], flags: NO_VARIANT_FLAGS }]
107                                                                                                                 flags: IS_ENUM
-                                                                                                                 repr: ReprOptions { int: None, align: None, pack: None, flags: (empty), field_shuffle_seed: 3477539199540094892 }
+                                                                                                                 repr: ReprOptions { int: None, align: None, pack: None, flags: (empty), field_shuffle_seed: [172, 103, 21, 41, 122, 179, 66, 48] }
109                                                                                                         substs: []
111                                                                                                         subpatterns: [


119                                                                                                                                 did: DefId(0:3 ~ thir_tree_match[fcf8]::Bar)
120                                                                                                                                 variants: [VariantDef { def_id: DefId(0:4 ~ thir_tree_match[fcf8]::Bar::First), ctor: Some((Const, DefId(0:5 ~ thir_tree_match[fcf8]::Bar::First::{constructor#0}))), name: "First", discr: Relative(0), fields: [], flags: NO_VARIANT_FLAGS }, VariantDef { def_id: DefId(0:6 ~ thir_tree_match[fcf8]::Bar::Second), ctor: Some((Const, DefId(0:7 ~ thir_tree_match[fcf8]::Bar::Second::{constructor#0}))), name: "Second", discr: Relative(1), fields: [], flags: NO_VARIANT_FLAGS }, VariantDef { def_id: DefId(0:8 ~ thir_tree_match[fcf8]::Bar::Third), ctor: Some((Const, DefId(0:9 ~ thir_tree_match[fcf8]::Bar::Third::{constructor#0}))), name: "Third", discr: Relative(2), fields: [], flags: NO_VARIANT_FLAGS }]
121                                                                                                                                 flags: IS_ENUM
-                                                                                                                                 repr: ReprOptions { int: None, align: None, pack: None, flags: (empty), field_shuffle_seed: 10333377570083945360 }
+                                                                                                                                 repr: ReprOptions { int: None, align: None, pack: None, flags: (empty), field_shuffle_seed: [144, 187, 215, 194, 38, 136, 103, 143] }
123                                                                                                                         substs: []
125                                                                                                                         subpatterns: []


178                                                                                                                 did: DefId(0:10 ~ thir_tree_match[fcf8]::Foo)
179                                                                                                                 variants: [VariantDef { def_id: DefId(0:11 ~ thir_tree_match[fcf8]::Foo::FooOne), ctor: Some((Fn, DefId(0:12 ~ thir_tree_match[fcf8]::Foo::FooOne::{constructor#0}))), name: "FooOne", discr: Relative(0), fields: [FieldDef { did: DefId(0:13 ~ thir_tree_match[fcf8]::Foo::FooOne::0), name: "0", vis: Restricted(DefId(0:0 ~ thir_tree_match[fcf8])) }], flags: NO_VARIANT_FLAGS }, VariantDef { def_id: DefId(0:14 ~ thir_tree_match[fcf8]::Foo::FooTwo), ctor: Some((Const, DefId(0:15 ~ thir_tree_match[fcf8]::Foo::FooTwo::{constructor#0}))), name: "FooTwo", discr: Relative(1), fields: [], flags: NO_VARIANT_FLAGS }]
180                                                                                                                 flags: IS_ENUM
-                                                                                                                 repr: ReprOptions { int: None, align: None, pack: None, flags: (empty), field_shuffle_seed: 3477539199540094892 }
+                                                                                                                 repr: ReprOptions { int: None, align: None, pack: None, flags: (empty), field_shuffle_seed: [172, 103, 21, 41, 122, 179, 66, 48] }
182                                                                                                         substs: []
184                                                                                                         subpatterns: [


241                                                                                                                 did: DefId(0:10 ~ thir_tree_match[fcf8]::Foo)
242                                                                                                                 variants: [VariantDef { def_id: DefId(0:11 ~ thir_tree_match[fcf8]::Foo::FooOne), ctor: Some((Fn, DefId(0:12 ~ thir_tree_match[fcf8]::Foo::FooOne::{constructor#0}))), name: "FooOne", discr: Relative(0), fields: [FieldDef { did: DefId(0:13 ~ thir_tree_match[fcf8]::Foo::FooOne::0), name: "0", vis: Restricted(DefId(0:0 ~ thir_tree_match[fcf8])) }], flags: NO_VARIANT_FLAGS }, VariantDef { def_id: DefId(0:14 ~ thir_tree_match[fcf8]::Foo::FooTwo), ctor: Some((Const, DefId(0:15 ~ thir_tree_match[fcf8]::Foo::FooTwo::{constructor#0}))), name: "FooTwo", discr: Relative(1), fields: [], flags: NO_VARIANT_FLAGS }]
243                                                                                                                 flags: IS_ENUM
-                                                                                                                 repr: ReprOptions { int: None, align: None, pack: None, flags: (empty), field_shuffle_seed: 3477539199540094892 }
+                                                                                                                 repr: ReprOptions { int: None, align: None, pack: None, flags: (empty), field_shuffle_seed: [172, 103, 21, 41, 122, 179, 66, 48] }
245                                                                                                         substs: []
247                                                                                                         subpatterns: []


The actual stdout differed from the expected stdout.
The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thir-print/thir-tree-match/thir-tree-match.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args thir-print/thir-tree-match.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/thir-print/thir-tree-match.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thir-print/thir-tree-match" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thir-print/thir-tree-match/auxiliary" "-Zunpretty=thir-tree"
--- stdout -------------------------------
DefId(0:16 ~ thir_tree_match[fcf8]::has_match):
params: [
    Param {
        ty: Foo
        ty_span: Some(fake-test-src-base/thir-print/thir-tree-match.rs:15:19: 15:22 (#0))
        self_kind: None
        hir_id: Some(HirId(DefId(0:16 ~ thir_tree_match[fcf8]::has_match).1))
        param: Some( 
            Pat: {
                ty: Foo
                span: fake-test-src-base/thir-print/thir-tree-match.rs:15:14: 15:17 (#0)
                kind: PatKind {
                        mutability: Not
                        name: "foo"
                        mode: ByValue
                        mode: ByValue
                        var: LocalVarId(HirId(DefId(0:16 ~ thir_tree_match[fcf8]::has_match).2))
                        ty: Foo
                        is_primary: true
                        subpattern: None
                }
            }
        )
    }
    }
]
body:
    Expr {
        ty: bool
        temp_lifetime: Some(Node(26))
        span: fake-test-src-base/thir-print/thir-tree-match.rs:15:32: 21:2 (#0)
            Scope {
                region_scope: Destruction(26)
                lint_level: Inherited
                value:
                value:
                    Expr {
                        ty: bool
                        temp_lifetime: Some(Node(26))
                        span: fake-test-src-base/thir-print/thir-tree-match.rs:15:32: 21:2 (#0)
                            Scope {
                                region_scope: Node(26)
                                region_scope: Node(26)
                                lint_level: Explicit(HirId(DefId(0:16 ~ thir_tree_match[fcf8]::has_match).26))
                                    Expr {
                                        ty: bool
                                        temp_lifetime: Some(Node(26))
                                        temp_lifetime: Some(Node(26))
                                        span: fake-test-src-base/thir-print/thir-tree-match.rs:15:32: 21:2 (#0)
                                            Block {
                                            Block {
                                                targeted_by_break: false
                                                opt_destruction_scope: None
                                                span: fake-test-src-base/thir-print/thir-tree-match.rs:15:32: 21:2 (#0)
                                                region_scope: Node(25)
                                                safety_mode: Safe
                                                stmts: []
                                                    Expr {
                                                        ty: bool
                                                        temp_lifetime: Some(Node(26))
                                                        temp_lifetime: Some(Node(26))
                                                        span: fake-test-src-base/thir-print/thir-tree-match.rs:16:5: 20:6 (#0)
                                                            Scope {
                                                                region_scope: Node(3)
                                                                region_scope: Node(3)
                                                                lint_level: Explicit(HirId(DefId(0:16 ~ thir_tree_match[fcf8]::has_match).3))
                                                                    Expr {
                                                                        ty: bool
                                                                        temp_lifetime: Some(Node(26))
                                                                        temp_lifetime: Some(Node(26))
                                                                        span: fake-test-src-base/thir-print/thir-tree-match.rs:16:5: 20:6 (#0)
                                                                            Match {
                                                                                scrutinee:
                                                                                    Expr {
                                                                                        ty: Foo
                                                                                        ty: Foo
                                                                                        temp_lifetime: Some(Node(26))
                                                                                        span: fake-test-src-base/thir-print/thir-tree-match.rs:16:11: 16:14 (#0)
                                                                                            Scope {
                                                                                                region_scope: Node(4)
                                                                                                region_scope: Node(4)
                                                                                                lint_level: Explicit(HirId(DefId(0:16 ~ thir_tree_match[fcf8]::has_match).4))
                                                                                                    Expr {
                                                                                                        ty: Foo
                                                                                                        temp_lifetime: Some(Node(26))
                                                                                                        temp_lifetime: Some(Node(26))
                                                                                                        span: fake-test-src-base/thir-print/thir-tree-match.rs:16:11: 16:14 (#0)
                                                                                                            VarRef {
                                                                                                            VarRef {
                                                                                                                id: LocalVarId(HirId(DefId(0:16 ~ thir_tree_match[fcf8]::has_match).2))
                                                                                                    }
                                                                                            }
                                                                                    }
                                                                                arms: [
                                                                                arms: [
                                                                                    Arm {
                                                                                        pattern: 
                                                                                            Pat: {
                                                                                                ty: Foo
                                                                                                span: fake-test-src-base/thir-print/thir-tree-match.rs:17:9: 17:32 (#0)
                                                                                                kind: PatKind {
                                                                                                    Variant {
                                                                                                        adt_def: 
                                                                                                            AdtDef {
                                                                                                                did: DefId(0:10 ~ thir_tree_match[fcf8]::Foo)
                                                                                                                variants: [VariantDef { def_id: DefId(0:11 ~ thir_tree_match[fcf8]::Foo::FooOne), ctor: Some((Fn, DefId(0:12 ~ thir_tree_match[fcf8]::Foo::FooOne::{constructor#0}))), name: "FooOne", discr: Relative(0), fields: [FieldDef { did: DefId(0:13 ~ thir_tree_match[fcf8]::Foo::FooOne::0), name: "0", vis: Restricted(DefId(0:0 ~ thir_tree_match[fcf8])) }], flags: NO_VARIANT_FLAGS }, VariantDef { def_id: DefId(0:14 ~ thir_tree_match[fcf8]::Foo::FooTwo), ctor: Some((Const, DefId(0:15 ~ thir_tree_match[fcf8]::Foo::FooTwo::{constructor#0}))), name: "FooTwo", discr: Relative(1), fields: [], flags: NO_VARIANT_FLAGS }]
                                                                                                                flags: IS_ENUM
                                                                                                                repr: ReprOptions { int: None, align: None, pack: None, flags: (empty), field_shuffle_seed: [172, 103, 21, 41, 122, 179, 66, 48] }
                                                                                                        substs: []
                                                                                                        subpatterns: [
                                                                                                            Pat: {
                                                                                                                ty: Bar
                                                                                                                ty: Bar
                                                                                                                span: fake-test-src-base/thir-print/thir-tree-match.rs:17:21: 17:31 (#0)
                                                                                                                kind: PatKind {
                                                                                                                    Variant {
                                                                                                                        adt_def: 
                                                                                                                            AdtDef {
                                                                                                                                did: DefId(0:3 ~ thir_tree_match[fcf8]::Bar)
                                                                                                                                variants: [VariantDef { def_id: DefId(0:4 ~ thir_tree_match[fcf8]::Bar::First), ctor: Some((Const, DefId(0:5 ~ thir_tree_match[fcf8]::Bar::First::{constructor#0}))), name: "First", discr: Relative(0), fields: [], flags: NO_VARIANT_FLAGS }, VariantDef { def_id: DefId(0:6 ~ thir_tree_match[fcf8]::Bar::Second), ctor: Some((Const, DefId(0:7 ~ thir_tree_match[fcf8]::Bar::Second::{constructor#0}))), name: "Second", discr: Relative(1), fields: [], flags: NO_VARIANT_FLAGS }, VariantDef { def_id: DefId(0:8 ~ thir_tree_match[fcf8]::Bar::Third), ctor: Some((Const, DefId(0:9 ~ thir_tree_match[fcf8]::Bar::Third::{constructor#0}))), name: "Third", discr: Relative(2), fields: [], flags: NO_VARIANT_FLAGS }]
                                                                                                                                flags: IS_ENUM
                                                                                                                                repr: ReprOptions { int: None, align: None, pack: None, flags: (empty), field_shuffle_seed: [144, 187, 215, 194, 38, 136, 103, 143] }
                                                                                                                        substs: []
                                                                                                                        subpatterns: []
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                            }
                                                                                                        ]
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        guard: None
                                                                                        body: 
                                                                                            Expr {
                                                                                                ty: bool
                                                                                                temp_lifetime: Some(Node(13))
                                                                                                span: fake-test-src-base/thir-print/thir-tree-match.rs:17:36: 17:40 (#0)
                                                                                                    Scope {
                                                                                                        region_scope: Destruction(13)
                                                                                                        lint_level: Inherited
                                                                                                        value:
                                                                                                        value:
                                                                                                            Expr {
                                                                                                                ty: bool
                                                                                                                temp_lifetime: Some(Node(13))
                                                                                                                span: fake-test-src-base/thir-print/thir-tree-match.rs:17:36: 17:40 (#0)
                                                                                                                    Scope {
                                                                                                                        region_scope: Node(13)
                                                                                                                        region_scope: Node(13)
                                                                                                                        lint_level: Explicit(HirId(DefId(0:16 ~ thir_tree_match[fcf8]::has_match).13))
                                                                                                                            Expr {
                                                                                                                                ty: bool
                                                                                                                                temp_lifetime: Some(Node(13))
                                                                                                                                temp_lifetime: Some(Node(13))
                                                                                                                                span: fake-test-src-base/thir-print/thir-tree-match.rs:17:36: 17:40 (#0)
                                                                                                                                kind: 
                                                                                                                                    Literal( lit: Spanned { node: Bool(true), span: fake-test-src-base/thir-print/thir-tree-match.rs:17:36: 17:40 (#0) }, neg: false)
                                                                                                                            }
                                                                                                                    }
                                                                                                            }
                                                                                                    }
                                                                                                    }
                                                                                            }
                                                                                        lint_level: Explicit(HirId(DefId(0:16 ~ thir_tree_match[fcf8]::has_match).12))
                                                                                        scope: Node(12)
                                                                                        span: fake-test-src-base/thir-print/thir-tree-match.rs:17:9: 17:40 (#0)
                                                                                    Arm {
                                                                                        pattern: 
                                                                                            Pat: {
                                                                                                ty: Foo
                                                                                                ty: Foo
                                                                                                span: fake-test-src-base/thir-print/thir-tree-match.rs:18:9: 18:23 (#0)
                                                                                                kind: PatKind {
                                                                                                    Variant {
                                                                                                        adt_def: 
                                                                                                            AdtDef {
                                                                                                                did: DefId(0:10 ~ thir_tree_match[fcf8]::Foo)
                                                                                                                variants: [VariantDef { def_id: DefId(0:11 ~ thir_tree_match[fcf8]::Foo::FooOne), ctor: Some((Fn, DefId(0:12 ~ thir_tree_match[fcf8]::Foo::FooOne::{constructor#0}))), name: "FooOne", discr: Relative(0), fields: [FieldDef { did: DefId(0:13 ~ thir_tree_match[fcf8]::Foo::FooOne::0), name: "0", vis: Restricted(DefId(0:0 ~ thir_tree_match[fcf8])) }], flags: NO_VARIANT_FLAGS }, VariantDef { def_id: DefId(0:14 ~ thir_tree_match[fcf8]::Foo::FooTwo), ctor: Some((Const, DefId(0:15 ~ thir_tree_match[fcf8]::Foo::FooTwo::{constructor#0}))), name: "FooTwo", discr: Relative(1), fields: [], flags: NO_VARIANT_FLAGS }]
                                                                                                                flags: IS_ENUM
                                                                                                                repr: ReprOptions { int: None, align: None, pack: None, flags: (empty), field_shuffle_seed: [172, 103, 21, 41, 122, 179, 66, 48] }
                                                                                                        substs: []
                                                                                                        subpatterns: [
                                                                                                            Pat: {
                                                                                                                ty: Bar
                                                                                                                ty: Bar
                                                                                                                span: fake-test-src-base/thir-print/thir-tree-match.rs:18:21: 18:22 (#0)
                                                                                                                kind: PatKind {
                                                                                                                    Wild
                                                                                                            }
                                                                                                        ]
                                                                                                    }
                                                                                                }
                                                                                                }
                                                                                            }
                                                                                        guard: None
                                                                                        body: 
                                                                                            Expr {
                                                                                                ty: bool
                                                                                                temp_lifetime: Some(Node(19))
                                                                                                span: fake-test-src-base/thir-print/thir-tree-match.rs:18:27: 18:32 (#0)
                                                                                                    Scope {
                                                                                                        region_scope: Destruction(19)
                                                                                                        lint_level: Inherited
                                                                                                        value:
                                                                                                        value:
                                                                                                            Expr {
                                                                                                                ty: bool
                                                                                                                temp_lifetime: Some(Node(19))
                                                                                                                span: fake-test-src-base/thir-print/thir-tree-match.rs:18:27: 18:32 (#0)
                                                                                                                    Scope {
                                                                                                                        region_scope: Node(19)
                                                                                                                        region_scope: Node(19)
                                                                                                                        lint_level: Explicit(HirId(DefId(0:16 ~ thir_tree_match[fcf8]::has_match).19))
                                                                                                                            Expr {
                                                                                                                                ty: bool
                                                                                                                                temp_lifetime: Some(Node(19))
                                                                                                                                temp_lifetime: Some(Node(19))
                                                                                                                                span: fake-test-src-base/thir-print/thir-tree-match.rs:18:27: 18:32 (#0)
                                                                                                                                kind: 
                                                                                                                                    Literal( lit: Spanned { node: Bool(false), span: fake-test-src-base/thir-print/thir-tree-match.rs:18:27: 18:32 (#0) }, neg: false)
                                                                                                                            }
                                                                                                                    }
                                                                                                            }
                                                                                                    }
                                                                                                    }
                                                                                            }
                                                                                        lint_level: Explicit(HirId(DefId(0:16 ~ thir_tree_match[fcf8]::has_match).18))
                                                                                        scope: Node(18)
                                                                                        span: fake-test-src-base/thir-print/thir-tree-match.rs:18:9: 18:32 (#0)
                                                                                    Arm {
                                                                                        pattern: 
                                                                                            Pat: {
                                                                                                ty: Foo
                                                                                                ty: Foo
                                                                                                span: fake-test-src-base/thir-print/thir-tree-match.rs:19:9: 19:20 (#0)
                                                                                                kind: PatKind {
                                                                                                    Variant {
                                                                                                        adt_def: 
                                                                                                            AdtDef {
                                                                                                                did: DefId(0:10 ~ thir_tree_match[fcf8]::Foo)
                                                                                                                variants: [VariantDef { def_id: DefId(0:11 ~ thir_tree_match[fcf8]::Foo::FooOne), ctor: Some((Fn, DefId(0:12 ~ thir_tree_match[fcf8]::Foo::FooOne::{constructor#0}))), name: "FooOne", discr: Relative(0), fields: [FieldDef { did: DefId(0:13 ~ thir_tree_match[fcf8]::Foo::FooOne::0), name: "0", vis: Restricted(DefId(0:0 ~ thir_tree_match[fcf8])) }], flags: NO_VARIANT_FLAGS }, VariantDef { def_id: DefId(0:14 ~ thir_tree_match[fcf8]::Foo::FooTwo), ctor: Some((Const, DefId(0:15 ~ thir_tree_match[fcf8]::Foo::FooTwo::{constructor#0}))), name: "FooTwo", discr: Relative(1), fields: [], flags: NO_VARIANT_FLAGS }]
                                                                                                                flags: IS_ENUM
                                                                                                                repr: ReprOptions { int: None, align: None, pack: None, flags: (empty), field_shuffle_seed: [172, 103, 21, 41, 122, 179, 66, 48] }
                                                                                                        substs: []
                                                                                                        subpatterns: []
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            }
                                                                                        guard: None
                                                                                        body: 
                                                                                            Expr {
                                                                                                ty: bool
                                                                                                temp_lifetime: Some(Node(24))
                                                                                                span: fake-test-src-base/thir-print/thir-tree-match.rs:19:24: 19:28 (#0)
                                                                                                    Scope {
                                                                                                        region_scope: Destruction(24)
                                                                                                        lint_level: Inherited
                                                                                                        value:
                                                                                                        value:
                                                                                                            Expr {
                                                                                                                ty: bool
                                                                                                                temp_lifetime: Some(Node(24))
                                                                                                                span: fake-test-src-base/thir-print/thir-tree-match.rs:19:24: 19:28 (#0)
                                                                                                                    Scope {
                                                                                                                        region_scope: Node(24)
                                                                                                                        region_scope: Node(24)
                                                                                                                        lint_level: Explicit(HirId(DefId(0:16 ~ thir_tree_match[fcf8]::has_match).24))
                                                                                                                            Expr {
                                                                                                                                ty: bool
                                                                                                                                temp_lifetime: Some(Node(24))
                                                                                                                                temp_lifetime: Some(Node(24))
                                                                                                                                span: fake-test-src-base/thir-print/thir-tree-match.rs:19:24: 19:28 (#0)
                                                                                                                                kind: 
                                                                                                                                    Literal( lit: Spanned { node: Bool(true), span: fake-test-src-base/thir-print/thir-tree-match.rs:19:24: 19:28 (#0) }, neg: false)
                                                                                                                            }
                                                                                                                    }
                                                                                                            }
                                                                                                    }
                                                                                                    }
                                                                                            }
                                                                                        lint_level: Explicit(HirId(DefId(0:16 ~ thir_tree_match[fcf8]::has_match).23))
                                                                                        scope: Node(23)
                                                                                        span: fake-test-src-base/thir-print/thir-tree-match.rs:19:9: 19:28 (#0)
                                                                                ]
                                                                            }
                                                                    }
                                                            }
---
            }
    }


DefId(0:17 ~ thir_tree_match[fcf8]::main):
params: [
body:
    Expr {
        ty: ()
        temp_lifetime: Some(Node(2))
        temp_lifetime: Some(Node(2))
        span: fake-test-src-base/thir-print/thir-tree-match.rs:23:11: 23:13 (#0)
            Scope {
                region_scope: Destruction(2)
                lint_level: Inherited
                value:
                value:
                    Expr {
                        ty: ()
                        temp_lifetime: Some(Node(2))
                        span: fake-test-src-base/thir-print/thir-tree-match.rs:23:11: 23:13 (#0)
                            Scope {
                                region_scope: Node(2)
                                region_scope: Node(2)
                                lint_level: Explicit(HirId(DefId(0:17 ~ thir_tree_match[fcf8]::main).2))
                                    Expr {
                                        ty: ()
                                        temp_lifetime: Some(Node(2))
                                        temp_lifetime: Some(Node(2))
                                        span: fake-test-src-base/thir-print/thir-tree-match.rs:23:11: 23:13 (#0)
                                            Block {
                                            Block {
                                                targeted_by_break: false
                                                opt_destruction_scope: None
                                                span: fake-test-src-base/thir-print/thir-tree-match.rs:23:11: 23:13 (#0)
                                                region_scope: Node(1)
                                                safety_mode: Safe
                                                stmts: []
                                                expr: []
                                    }
                            }
                    }
            }
