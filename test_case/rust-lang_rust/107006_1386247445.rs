plain
...........i.ii......................................................................... 14168/14202
..................................
failures:

---- [ui] checkout/tests/ui/thir-tree-match.rs stdout ----

59                                                     Expr {
60                                                         ty: bool
61                                                         temp_lifetime: Some(Node(26))
61                                                         temp_lifetime: Some(Node(26))
-                                                         span: $DIR/thir-tree-match.rs:16:3: 20:4 (#0)
+                                                         span: $DIR/thir-tree-match.rs:16:5: 20:6 (#0)
64                                                             Scope {
65                                                                 region_scope: Node(3)

68                                                                     Expr {
68                                                                     Expr {
69                                                                         ty: bool
70                                                                         temp_lifetime: Some(Node(26))
-                                                                         span: $DIR/thir-tree-match.rs:16:3: 20:4 (#0)
+                                                                         span: $DIR/thir-tree-match.rs:16:5: 20:6 (#0)
73                                                                             Match {
74                                                                                 scrutinee:

75                                                                                     Expr {
75                                                                                     Expr {
76                                                                                         ty: Foo
77                                                                                         temp_lifetime: Some(Node(26))
-                                                                                         span: $DIR/thir-tree-match.rs:16:9: 16:12 (#0)
+                                                                                         span: $DIR/thir-tree-match.rs:16:11: 16:14 (#0)
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
80                                                                                             Scope {
81                                                                                                 region_scope: Node(4)


84                                                                                                     Expr {
85                                                                                                         ty: Foo
86                                                                                                         temp_lifetime: Some(Node(26))
-                                                                                                         span: $DIR/thir-tree-match.rs:16:9: 16:12 (#0)
+                                                                                                         span: $DIR/thir-tree-match.rs:16:11: 16:14 (#0)
89                                                                                                             VarRef {
89                                                                                                             VarRef {
90                                                                                                                 id: LocalVarId(HirId(DefId(0:16 ~ thir_tree_match[3c9a]::has_match).2))
97                                                                                         pattern: 
98                                                                                             Pat: {
99                                                                                                 ty: Foo
99                                                                                                 ty: Foo
-                                                                                                 span: $DIR/thir-tree-match.rs:17:5: 17:28 (#0)
+                                                                                                 span: $DIR/thir-tree-match.rs:17:9: 17:32 (#0)
101                                                                                                 kind: PatKind {
102                                                                                                     Variant {
103                                                                                                         adt_def: 
111                                                                                                         subpatterns: [
112                                                                                                             Pat: {
113                                                                                                                 ty: Bar
113                                                                                                                 ty: Bar
-                                                                                                                 span: $DIR/thir-tree-match.rs:17:17: 17:27 (#0)
+                                                                                                                 span: $DIR/thir-tree-match.rs:17:21: 17:31 (#0)
115                                                                                                                 kind: PatKind {
116                                                                                                                     Variant {
117                                                                                                                         adt_def: 
135                                                                                             Expr {
136                                                                                                 ty: bool
137                                                                                                 temp_lifetime: Some(Node(13))
137                                                                                                 temp_lifetime: Some(Node(13))
-                                                                                                 span: $DIR/thir-tree-match.rs:17:32: 17:36 (#0)
+                                                                                                 span: $DIR/thir-tree-match.rs:17:36: 17:40 (#0)
140                                                                                                     Scope {
141                                                                                                         region_scope: Destruction(13)

144                                                                                                             Expr {
144                                                                                                             Expr {
145                                                                                                                 ty: bool
146                                                                                                                 temp_lifetime: Some(Node(13))
-                                                                                                                 span: $DIR/thir-tree-match.rs:17:32: 17:36 (#0)
+                                                                                                                 span: $DIR/thir-tree-match.rs:17:36: 17:40 (#0)
149                                                                                                                     Scope {
150                                                                                                                         region_scope: Node(13)

153                                                                                                                             Expr {
153                                                                                                                             Expr {
154                                                                                                                                 ty: bool
155                                                                                                                                 temp_lifetime: Some(Node(13))
-                                                                                                                                 span: $DIR/thir-tree-match.rs:17:32: 17:36 (#0)
+                                                                                                                                 span: $DIR/thir-tree-match.rs:17:36: 17:40 (#0)
157                                                                                                                                 kind: 
-                                                                                                                                     Literal( lit: Spanned { node: Bool(true), span: $DIR/thir-tree-match.rs:17:32: 17:36 (#0) }, neg: false)
+                                                                                                                                     Literal( lit: Spanned { node: Bool(true), span: $DIR/thir-tree-match.rs:17:36: 17:40 (#0) }, neg: false)
160                                                                                                                             }
161                                                                                                                     }

164                                                                                             }
164                                                                                             }
165                                                                                         lint_level: Explicit(HirId(DefId(0:16 ~ thir_tree_match[3c9a]::has_match).12))
166                                                                                         scope: Node(12)
-                                                                                         span: $DIR/thir-tree-match.rs:17:5: 17:36 (#0)
+                                                                                         span: $DIR/thir-tree-match.rs:17:9: 17:40 (#0)
169                                                                                     Arm {
170                                                                                         pattern: 

171                                                                                             Pat: {
171                                                                                             Pat: {
172                                                                                                 ty: Foo
-                                                                                                 span: $DIR/thir-tree-match.rs:18:5: 18:19 (#0)
+                                                                                                 span: $DIR/thir-tree-match.rs:18:9: 18:23 (#0)
174                                                                                                 kind: PatKind {
175                                                                                                     Variant {
176                                                                                                         adt_def: 
184                                                                                                         subpatterns: [
185                                                                                                             Pat: {
186                                                                                                                 ty: Bar
186                                                                                                                 ty: Bar
-                                                                                                                 span: $DIR/thir-tree-match.rs:18:17: 18:18 (#0)
+                                                                                                                 span: $DIR/thir-tree-match.rs:18:21: 18:22 (#0)
188                                                                                                                 kind: PatKind {
189                                                                                                                     Wild

198                                                                                             Expr {
199                                                                                                 ty: bool
200                                                                                                 temp_lifetime: Some(Node(19))
200                                                                                                 temp_lifetime: Some(Node(19))
-                                                                                                 span: $DIR/thir-tree-match.rs:18:23: 18:28 (#0)
+                                                                                                 span: $DIR/thir-tree-match.rs:18:27: 18:32 (#0)
203                                                                                                     Scope {
204                                                                                                         region_scope: Destruction(19)

207                                                                                                             Expr {
207                                                                                                             Expr {
208                                                                                                                 ty: bool
209                                                                                                                 temp_lifetime: Some(Node(19))
-                                                                                                                 span: $DIR/thir-tree-match.rs:18:23: 18:28 (#0)
+                                                                                                                 span: $DIR/thir-tree-match.rs:18:27: 18:32 (#0)
212                                                                                                                     Scope {
213                                                                                                                         region_scope: Node(19)

216                                                                                                                             Expr {
216                                                                                                                             Expr {
217                                                                                                                                 ty: bool
218                                                                                                                                 temp_lifetime: Some(Node(19))
-                                                                                                                                 span: $DIR/thir-tree-match.rs:18:23: 18:28 (#0)
+                                                                                                                                 span: $DIR/thir-tree-match.rs:18:27: 18:32 (#0)
220                                                                                                                                 kind: 
-                                                                                                                                     Literal( lit: Spanned { node: Bool(false), span: $DIR/thir-tree-match.rs:18:23: 18:28 (#0) }, neg: false)
+                                                                                                                                     Literal( lit: Spanned { node: Bool(false), span: $DIR/thir-tree-match.rs:18:27: 18:32 (#0) }, neg: false)
223                                                                                                                             }
224                                                                                                                     }

227                                                                                             }
227                                                                                             }
228                                                                                         lint_level: Explicit(HirId(DefId(0:16 ~ thir_tree_match[3c9a]::has_match).18))
229                                                                                         scope: Node(18)
-                                                                                         span: $DIR/thir-tree-match.rs:18:5: 18:28 (#0)
+                                                                                         span: $DIR/thir-tree-match.rs:18:9: 18:32 (#0)
232                                                                                     Arm {
233                                                                                         pattern: 

234                                                                                             Pat: {
234                                                                                             Pat: {
235                                                                                                 ty: Foo
-                                                                                                 span: $DIR/thir-tree-match.rs:19:5: 19:16 (#0)
+                                                                                                 span: $DIR/thir-tree-match.rs:19:9: 19:20 (#0)
237                                                                                                 kind: PatKind {
238                                                                                                     Variant {
239                                                                                                         adt_def: 
253                                                                                             Expr {
254                                                                                                 ty: bool
255                                                                                                 temp_lifetime: Some(Node(24))
255                                                                                                 temp_lifetime: Some(Node(24))
-                                                                                                 span: $DIR/thir-tree-match.rs:19:20: 19:24 (#0)
+                                                                                                 span: $DIR/thir-tree-match.rs:19:24: 19:28 (#0)
258                                                                                                     Scope {
259                                                                                                         region_scope: Destruction(24)

262                                                                                                             Expr {
262                                                                                                             Expr {
263                                                                                                                 ty: bool
264                                                                                                                 temp_lifetime: Some(Node(24))
-                                                                                                                 span: $DIR/thir-tree-match.rs:19:20: 19:24 (#0)
+                                                                                                                 span: $DIR/thir-tree-match.rs:19:24: 19:28 (#0)
267                                                                                                                     Scope {
268                                                                                                                         region_scope: Node(24)

271                                                                                                                             Expr {
271                                                                                                                             Expr {
272                                                                                                                                 ty: bool
273                                                                                                                                 temp_lifetime: Some(Node(24))
-                                                                                                                                 span: $DIR/thir-tree-match.rs:19:20: 19:24 (#0)
+                                                                                                                                 span: $DIR/thir-tree-match.rs:19:24: 19:28 (#0)
275                                                                                                                                 kind: 
-                                                                                                                                     Literal( lit: Spanned { node: Bool(true), span: $DIR/thir-tree-match.rs:19:20: 19:24 (#0) }, neg: false)
+                                                                                                                                     Literal( lit: Spanned { node: Bool(true), span: $DIR/thir-tree-match.rs:19:24: 19:28 (#0) }, neg: false)
278                                                                                                                             }
279                                                                                                                     }

282                                                                                             }
282                                                                                             }
283                                                                                         lint_level: Explicit(HirId(DefId(0:16 ~ thir_tree_match[3c9a]::has_match).23))
284                                                                                         scope: Node(23)
-                                                                                         span: $DIR/thir-tree-match.rs:19:5: 19:24 (#0)
+                                                                                         span: $DIR/thir-tree-match.rs:19:9: 19:28 (#0)
287                                                                                 ]
288                                                                             }



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thir-tree-match/thir-tree-match.stdout
To only update this specific test, also pass `--test-args thir-tree-match.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/thir-tree-match.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thir-tree-match" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thir-tree-match/auxiliary" "-Zunpretty=thir-tree"
--- stdout -------------------------------
DefId(0:16 ~ thir_tree_match[3c9a]::has_match):
params: [
    Param {
        ty: Foo
        ty_span: Some(/checkout/tests/ui/thir-tree-match.rs:15:19: 15:22 (#0))
        self_kind: None
        hir_id: Some(HirId(DefId(0:16 ~ thir_tree_match[3c9a]::has_match).1))
        param: Some( 
            Pat: {
                ty: Foo
                span: /checkout/tests/ui/thir-tree-match.rs:15:14: 15:17 (#0)
                kind: PatKind {
                        mutability: Not
                        name: "foo"
                        mode: ByValue
                        mode: ByValue
                        var: LocalVarId(HirId(DefId(0:16 ~ thir_tree_match[3c9a]::has_match).2))
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
        span: /checkout/tests/ui/thir-tree-match.rs:15:32: 21:2 (#0)
            Scope {
                region_scope: Destruction(26)
                lint_level: Inherited
                value:
                value:
                    Expr {
                        ty: bool
                        temp_lifetime: Some(Node(26))
                        span: /checkout/tests/ui/thir-tree-match.rs:15:32: 21:2 (#0)
                            Scope {
                                region_scope: Node(26)
                                region_scope: Node(26)
                                lint_level: Explicit(HirId(DefId(0:16 ~ thir_tree_match[3c9a]::has_match).26))
                                    Expr {
                                        ty: bool
                                        temp_lifetime: Some(Node(26))
                                        temp_lifetime: Some(Node(26))
                                        span: /checkout/tests/ui/thir-tree-match.rs:15:32: 21:2 (#0)
                                            Block {
                                            Block {
                                                targeted_by_break: false
                                                opt_destruction_scope: None
                                                span: /checkout/tests/ui/thir-tree-match.rs:15:32: 21:2 (#0)
                                                region_scope: Node(25)
                                                safety_mode: Safe
                                                stmts: []
                                                    Expr {
                                                        ty: bool
                                                        temp_lifetime: Some(Node(26))
                                                        temp_lifetime: Some(Node(26))
                                                        span: /checkout/tests/ui/thir-tree-match.rs:16:5: 20:6 (#0)
                                                            Scope {
                                                                region_scope: Node(3)
                                                                region_scope: Node(3)
                                                                lint_level: Explicit(HirId(DefId(0:16 ~ thir_tree_match[3c9a]::has_match).3))
                                                                    Expr {
                                                                        ty: bool
                                                                        temp_lifetime: Some(Node(26))
                                                                        temp_lifetime: Some(Node(26))
                                                                        span: /checkout/tests/ui/thir-tree-match.rs:16:5: 20:6 (#0)
                                                                            Match {
                                                                                scrutinee:
                                                                                    Expr {
                                                                                        ty: Foo
                                                                                        ty: Foo
                                                                                        temp_lifetime: Some(Node(26))
                                                                                        span: /checkout/tests/ui/thir-tree-match.rs:16:11: 16:14 (#0)
                                                                                            Scope {
                                                                                                region_scope: Node(4)
                                                                                                region_scope: Node(4)
                                                                                                lint_level: Explicit(HirId(DefId(0:16 ~ thir_tree_match[3c9a]::has_match).4))
                                                                                                    Expr {
                                                                                                        ty: Foo
                                                                                                        temp_lifetime: Some(Node(26))
                                                                                                        temp_lifetime: Some(Node(26))
                                                                                                        span: /checkout/tests/ui/thir-tree-match.rs:16:11: 16:14 (#0)
                                                                                                            VarRef {
                                                                                                            VarRef {
                                                                                                                id: LocalVarId(HirId(DefId(0:16 ~ thir_tree_match[3c9a]::has_match).2))
                                                                                                    }
                                                                                            }
                                                                                    }
                                                                                arms: [
                                                                                arms: [
                                                                                    Arm {
                                                                                        pattern: 
                                                                                            Pat: {
                                                                                                ty: Foo
                                                                                                span: /checkout/tests/ui/thir-tree-match.rs:17:9: 17:32 (#0)
                                                                                                kind: PatKind {
                                                                                                    Variant {
                                                                                                        adt_def: 
                                                                                                            AdtDef {
                                                                                                                did: DefId(0:10 ~ thir_tree_match[3c9a]::Foo)
                                                                                                                variants: [VariantDef { def_id: DefId(0:11 ~ thir_tree_match[3c9a]::Foo::FooOne), ctor: Some((Fn, DefId(0:12 ~ thir_tree_match[3c9a]::Foo::FooOne::{constructor#0}))), name: "FooOne", discr: Relative(0), fields: [FieldDef { did: DefId(0:13 ~ thir_tree_match[3c9a]::Foo::FooOne::0), name: "0", vis: Restricted(DefId(0:0 ~ thir_tree_match[3c9a])) }], flags: NO_VARIANT_FLAGS }, VariantDef { def_id: DefId(0:14 ~ thir_tree_match[3c9a]::Foo::FooTwo), ctor: Some((Const, DefId(0:15 ~ thir_tree_match[3c9a]::Foo::FooTwo::{constructor#0}))), name: "FooTwo", discr: Relative(1), fields: [], flags: NO_VARIANT_FLAGS }]
                                                                                                                flags: IS_ENUM
                                                                                                                repr: ReprOptions { int: None, align: None, pack: None, flags: (empty), field_shuffle_seed: 11573694388057581 }
                                                                                                        substs: []
                                                                                                        subpatterns: [
                                                                                                            Pat: {
                                                                                                                ty: Bar
                                                                                                                ty: Bar
                                                                                                                span: /checkout/tests/ui/thir-tree-match.rs:17:21: 17:31 (#0)
                                                                                                                kind: PatKind {
                                                                                                                    Variant {
                                                                                                                        adt_def: 
                                                                                                                            AdtDef {
                                                                                                                                did: DefId(0:3 ~ thir_tree_match[3c9a]::Bar)
                                                                                                                                variants: [VariantDef { def_id: DefId(0:4 ~ thir_tree_match[3c9a]::Bar::First), ctor: Some((Const, DefId(0:5 ~ thir_tree_match[3c9a]::Bar::First::{constructor#0}))), name: "First", discr: Relative(0), fields: [], flags: NO_VARIANT_FLAGS }, VariantDef { def_id: DefId(0:6 ~ thir_tree_match[3c9a]::Bar::Second), ctor: Some((Const, DefId(0:7 ~ thir_tree_match[3c9a]::Bar::Second::{constructor#0}))), name: "Second", discr: Relative(1), fields: [], flags: NO_VARIANT_FLAGS }, VariantDef { def_id: DefId(0:8 ~ thir_tree_match[3c9a]::Bar::Third), ctor: Some((Const, DefId(0:9 ~ thir_tree_match[3c9a]::Bar::Third::{constructor#0}))), name: "Third", discr: Relative(2), fields: [], flags: NO_VARIANT_FLAGS }]
                                                                                                                                flags: IS_ENUM
                                                                                                                                repr: ReprOptions { int: None, align: None, pack: None, flags: (empty), field_shuffle_seed: 3125160937860410723 }
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
                                                                                                span: /checkout/tests/ui/thir-tree-match.rs:17:36: 17:40 (#0)
                                                                                                    Scope {
                                                                                                        region_scope: Destruction(13)
                                                                                                        lint_level: Inherited
                                                                                                        value:
                                                                                                        value:
                                                                                                            Expr {
                                                                                                                ty: bool
                                                                                                                temp_lifetime: Some(Node(13))
                                                                                                                span: /checkout/tests/ui/thir-tree-match.rs:17:36: 17:40 (#0)
                                                                                                                    Scope {
                                                                                                                        region_scope: Node(13)
                                                                                                                        region_scope: Node(13)
                                                                                                                        lint_level: Explicit(HirId(DefId(0:16 ~ thir_tree_match[3c9a]::has_match).13))
                                                                                                                            Expr {
                                                                                                                                ty: bool
                                                                                                                                temp_lifetime: Some(Node(13))
                                                                                                                                temp_lifetime: Some(Node(13))
                                                                                                                                span: /checkout/tests/ui/thir-tree-match.rs:17:36: 17:40 (#0)
                                                                                                                                kind: 
                                                                                                                                    Literal( lit: Spanned { node: Bool(true), span: /checkout/tests/ui/thir-tree-match.rs:17:36: 17:40 (#0) }, neg: false)
                                                                                                                            }
                                                                                                                    }
                                                                                                            }
                                                                                                    }
                                                                                                    }
                                                                                            }
                                                                                        lint_level: Explicit(HirId(DefId(0:16 ~ thir_tree_match[3c9a]::has_match).12))
                                                                                        scope: Node(12)
                                                                                        span: /checkout/tests/ui/thir-tree-match.rs:17:9: 17:40 (#0)
                                                                                    Arm {
                                                                                        pattern: 
                                                                                            Pat: {
                                                                                                ty: Foo
                                                                                                ty: Foo
                                                                                                span: /checkout/tests/ui/thir-tree-match.rs:18:9: 18:23 (#0)
                                                                                                kind: PatKind {
                                                                                                    Variant {
                                                                                                        adt_def: 
                                                                                                            AdtDef {
                                                                                                                did: DefId(0:10 ~ thir_tree_match[3c9a]::Foo)
                                                                                                                variants: [VariantDef { def_id: DefId(0:11 ~ thir_tree_match[3c9a]::Foo::FooOne), ctor: Some((Fn, DefId(0:12 ~ thir_tree_match[3c9a]::Foo::FooOne::{constructor#0}))), name: "FooOne", discr: Relative(0), fields: [FieldDef { did: DefId(0:13 ~ thir_tree_match[3c9a]::Foo::FooOne::0), name: "0", vis: Restricted(DefId(0:0 ~ thir_tree_match[3c9a])) }], flags: NO_VARIANT_FLAGS }, VariantDef { def_id: DefId(0:14 ~ thir_tree_match[3c9a]::Foo::FooTwo), ctor: Some((Const, DefId(0:15 ~ thir_tree_match[3c9a]::Foo::FooTwo::{constructor#0}))), name: "FooTwo", discr: Relative(1), fields: [], flags: NO_VARIANT_FLAGS }]
                                                                                                                flags: IS_ENUM
                                                                                                                repr: ReprOptions { int: None, align: None, pack: None, flags: (empty), field_shuffle_seed: 11573694388057581 }
                                                                                                        substs: []
                                                                                                        subpatterns: [
                                                                                                            Pat: {
                                                                                                                ty: Bar
                                                                                                                ty: Bar
                                                                                                                span: /checkout/tests/ui/thir-tree-match.rs:18:21: 18:22 (#0)
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
                                                                                                span: /checkout/tests/ui/thir-tree-match.rs:18:27: 18:32 (#0)
                                                                                                    Scope {
                                                                                                        region_scope: Destruction(19)
                                                                                                        lint_level: Inherited
                                                                                                        value:
                                                                                                        value:
                                                                                                            Expr {
                                                                                                                ty: bool
                                                                                                                temp_lifetime: Some(Node(19))
                                                                                                                span: /checkout/tests/ui/thir-tree-match.rs:18:27: 18:32 (#0)
                                                                                                                    Scope {
                                                                                                                        region_scope: Node(19)
                                                                                                                        region_scope: Node(19)
                                                                                                                        lint_level: Explicit(HirId(DefId(0:16 ~ thir_tree_match[3c9a]::has_match).19))
                                                                                                                            Expr {
                                                                                                                                ty: bool
                                                                                                                                temp_lifetime: Some(Node(19))
                                                                                                                                temp_lifetime: Some(Node(19))
                                                                                                                                span: /checkout/tests/ui/thir-tree-match.rs:18:27: 18:32 (#0)
                                                                                                                                kind: 
                                                                                                                                    Literal( lit: Spanned { node: Bool(false), span: /checkout/tests/ui/thir-tree-match.rs:18:27: 18:32 (#0) }, neg: false)
                                                                                                                            }
                                                                                                                    }
                                                                                                            }
                                                                                                    }
                                                                                                    }
                                                                                            }
                                                                                        lint_level: Explicit(HirId(DefId(0:16 ~ thir_tree_match[3c9a]::has_match).18))
                                                                                        scope: Node(18)
                                                                                        span: /checkout/tests/ui/thir-tree-match.rs:18:9: 18:32 (#0)
                                                                                    Arm {
                                                                                        pattern: 
                                                                                            Pat: {
                                                                                                ty: Foo
                                                                                                ty: Foo
                                                                                                span: /checkout/tests/ui/thir-tree-match.rs:19:9: 19:20 (#0)
                                                                                                kind: PatKind {
                                                                                                    Variant {
                                                                                                        adt_def: 
                                                                                                            AdtDef {
                                                                                                                did: DefId(0:10 ~ thir_tree_match[3c9a]::Foo)
                                                                                                                variants: [VariantDef { def_id: DefId(0:11 ~ thir_tree_match[3c9a]::Foo::FooOne), ctor: Some((Fn, DefId(0:12 ~ thir_tree_match[3c9a]::Foo::FooOne::{constructor#0}))), name: "FooOne", discr: Relative(0), fields: [FieldDef { did: DefId(0:13 ~ thir_tree_match[3c9a]::Foo::FooOne::0), name: "0", vis: Restricted(DefId(0:0 ~ thir_tree_match[3c9a])) }], flags: NO_VARIANT_FLAGS }, VariantDef { def_id: DefId(0:14 ~ thir_tree_match[3c9a]::Foo::FooTwo), ctor: Some((Const, DefId(0:15 ~ thir_tree_match[3c9a]::Foo::FooTwo::{constructor#0}))), name: "FooTwo", discr: Relative(1), fields: [], flags: NO_VARIANT_FLAGS }]
                                                                                                                flags: IS_ENUM
                                                                                                                repr: ReprOptions { int: None, align: None, pack: None, flags: (empty), field_shuffle_seed: 11573694388057581 }
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
                                                                                                span: /checkout/tests/ui/thir-tree-match.rs:19:24: 19:28 (#0)
                                                                                                    Scope {
                                                                                                        region_scope: Destruction(24)
                                                                                                        lint_level: Inherited
                                                                                                        value:
                                                                                                        value:
                                                                                                            Expr {
                                                                                                                ty: bool
                                                                                                                temp_lifetime: Some(Node(24))
                                                                                                                span: /checkout/tests/ui/thir-tree-match.rs:19:24: 19:28 (#0)
                                                                                                                    Scope {
                                                                                                                        region_scope: Node(24)
                                                                                                                        region_scope: Node(24)
                                                                                                                        lint_level: Explicit(HirId(DefId(0:16 ~ thir_tree_match[3c9a]::has_match).24))
                                                                                                                            Expr {
                                                                                                                                ty: bool
                                                                                                                                temp_lifetime: Some(Node(24))
                                                                                                                                temp_lifetime: Some(Node(24))
                                                                                                                                span: /checkout/tests/ui/thir-tree-match.rs:19:24: 19:28 (#0)
                                                                                                                                kind: 
                                                                                                                                    Literal( lit: Spanned { node: Bool(true), span: /checkout/tests/ui/thir-tree-match.rs:19:24: 19:28 (#0) }, neg: false)
                                                                                                                            }
                                                                                                                    }
                                                                                                            }
                                                                                                    }
                                                                                                    }
                                                                                            }
                                                                                        lint_level: Explicit(HirId(DefId(0:16 ~ thir_tree_match[3c9a]::has_match).23))
                                                                                        scope: Node(23)
                                                                                        span: /checkout/tests/ui/thir-tree-match.rs:19:9: 19:28 (#0)
                                                                                ]
---
            }
    }


DefId(0:17 ~ thir_tree_match[3c9a]::main):
params: [
body:
    Expr {
        ty: ()
        temp_lifetime: Some(Node(2))
        temp_lifetime: Some(Node(2))
        span: /checkout/tests/ui/thir-tree-match.rs:23:11: 23:13 (#0)
            Scope {
                region_scope: Destruction(2)
                lint_level: Inherited
                value:
                value:
                    Expr {
                        ty: ()
                        temp_lifetime: Some(Node(2))
                        span: /checkout/tests/ui/thir-tree-match.rs:23:11: 23:13 (#0)
                            Scope {
                                region_scope: Node(2)
                                region_scope: Node(2)
                                lint_level: Explicit(HirId(DefId(0:17 ~ thir_tree_match[3c9a]::main).2))
                                    Expr {
                                        ty: ()
                                        temp_lifetime: Some(Node(2))
                                        temp_lifetime: Some(Node(2))
                                        span: /checkout/tests/ui/thir-tree-match.rs:23:11: 23:13 (#0)
                                            Block {
                                            Block {
                                                targeted_by_break: false
                                                opt_destruction_scope: None
                                                span: /checkout/tests/ui/thir-tree-match.rs:23:11: 23:13 (#0)
                                                region_scope: Node(1)
                                                safety_mode: Safe
                                                stmts: []
                                                expr: []
                                    }
                            }
                    }
            }
