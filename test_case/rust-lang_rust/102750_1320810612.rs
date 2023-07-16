plain

---- [ui] src/test/ui/stats/hir-stats.rs stdout ----
diff of stderr:

2 ast-stats-1 Name                Accumulated Size         Count     Item Size
3 ast-stats-1 ----------------------------------------------------------------
4 ast-stats-1 ExprField                 48 ( 0.6%)             1            48
- ast-stats-1 GenericArgs               56 ( 0.8%)             1            56
- ast-stats-1 - AngleBracketed            56 ( 0.8%)             1
7 ast-stats-1 Crate                     56 ( 0.8%)             1            56
8 ast-stats-1 Attribute                 64 ( 0.9%)             2            32
9 ast-stats-1 - Normal                    32 ( 0.4%)             1

10 ast-stats-1 - DocComment                32 ( 0.4%)             1
+ ast-stats-1 GenericArgs               64 ( 0.9%)             1            64
+ ast-stats-1 - AngleBracketed            64 ( 0.9%)             1
11 ast-stats-1 Local                     72 ( 1.0%)             1            72
12 ast-stats-1 WherePredicate            72 ( 1.0%)             1            72
13 ast-stats-1 - BoundPredicate            72 ( 1.0%)             1

53 ast-stats-1 - Fn                       368 ( 5.0%)             2
54 ast-stats-1 - Use                      552 ( 7.4%)             3
55 ast-stats-1 ----------------------------------------------------------------
- ast-stats-1 Total                  7_416
+ ast-stats-1 Total                  7_424
57 ast-stats-1
58 ast-stats-2 POST EXPANSION AST STATS
59 ast-stats-2 Name                Accumulated Size         Count     Item Size
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
60 ast-stats-2 ----------------------------------------------------------------
60 ast-stats-2 ----------------------------------------------------------------
61 ast-stats-2 ExprField                 48 ( 0.6%)             1            48
- ast-stats-2 GenericArgs               56 ( 0.7%)             1            56
- ast-stats-2 - AngleBracketed            56 ( 0.7%)             1
64 ast-stats-2 Crate                     56 ( 0.7%)             1            56
+ ast-stats-2 GenericArgs               64 ( 0.8%)             1            64
+ ast-stats-2 - AngleBracketed            64 ( 0.8%)             1
65 ast-stats-2 Local                     72 ( 0.9%)             1            72
66 ast-stats-2 WherePredicate            72 ( 0.9%)             1            72
67 ast-stats-2 - BoundPredicate            72 ( 0.9%)             1

80 ast-stats-2 Param                    160 ( 2.0%)             4            40
81 ast-stats-2 FnDecl                   200 ( 2.5%)             5            40
82 ast-stats-2 Variant                  240 ( 3.0%)             2           120
- ast-stats-2 GenericBound             288 ( 3.6%)             4            72
- ast-stats-2 - Trait                    288 ( 3.6%)             4
- ast-stats-2 Block                    288 ( 3.6%)             6            48
+ ast-stats-2 GenericBound             288 ( 3.5%)             4            72
+ ast-stats-2 - Trait                    288 ( 3.5%)             4
+ ast-stats-2 Block                    288 ( 3.5%)             6            48
86 ast-stats-2 AssocItem                416 ( 5.1%)             4           104
87 ast-stats-2 - Type                     208 ( 2.6%)             2
88 ast-stats-2 - Fn                       208 ( 2.6%)             2

104 ast-stats-2 - Ptr                       64 ( 0.8%)             1
105 ast-stats-2 - ImplicitSelf             128 ( 1.6%)             2
106 ast-stats-2 - Path                     640 ( 7.9%)            10
- ast-stats-2 Item                   2_024 (25.0%)            11           184
+ ast-stats-2 Item                   2_024 (24.9%)            11           184
108 ast-stats-2 - Trait                    184 ( 2.3%)             1
109 ast-stats-2 - Enum                     184 ( 2.3%)             1
110 ast-stats-2 - ExternCrate              184 ( 2.3%)             1

113 ast-stats-2 - Fn                       368 ( 4.5%)             2
114 ast-stats-2 - Use                      736 ( 9.1%)             4
115 ast-stats-2 ----------------------------------------------------------------
- ast-stats-2 Total                  8_112
+ ast-stats-2 Total                  8_120
117 ast-stats-2
118 hir-stats HIR STATS
119 hir-stats Name                Accumulated Size         Count     Item Size

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stats/hir-stats/hir-stats.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args stats/hir-stats.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/checkout/src/test/ui/stats/hir-stats.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stats/hir-stats" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stats/hir-stats/auxiliary" "-Zhir-stats"
stdout: none
--- stderr -------------------------------
ast-stats-1 PRE EXPANSION AST STATS
ast-stats-1 Name                Accumulated Size         Count     Item Size
ast-stats-1 ----------------------------------------------------------------
ast-stats-1 ExprField                 48 ( 0.6%)             1            48
ast-stats-1 Crate                     56 ( 0.8%)             1            56
ast-stats-1 Attribute                 64 ( 0.9%)             2            32
ast-stats-1 - Normal                    32 ( 0.4%)             1
ast-stats-1 - DocComment                32 ( 0.4%)             1
ast-stats-1 GenericArgs               64 ( 0.9%)             1            64
ast-stats-1 - AngleBracketed            64 ( 0.9%)             1
ast-stats-1 Local                     72 ( 1.0%)             1            72
ast-stats-1 WherePredicate            72 ( 1.0%)             1            72
ast-stats-1 - BoundPredicate            72 ( 1.0%)             1
ast-stats-1 Arm                       96 ( 1.3%)             2            48
ast-stats-1 ForeignItem               96 ( 1.3%)             1            96
ast-stats-1 - Fn                        96 ( 1.3%)             1
ast-stats-1 FieldDef                 160 ( 2.2%)             2            80
ast-stats-1 Stmt                     160 ( 2.2%)             5            32
ast-stats-1 - Local                     32 ( 0.4%)             1
ast-stats-1 - MacCall                   32 ( 0.4%)             1
ast-stats-1 - Expr                      96 ( 1.3%)             3
ast-stats-1 Param                    160 ( 2.2%)             4            40
ast-stats-1 FnDecl                   200 ( 2.7%)             5            40
ast-stats-1 Variant                  240 ( 3.2%)             2           120
ast-stats-1 GenericBound             288 ( 3.9%)             4            72
ast-stats-1 - Trait                    288 ( 3.9%)             4
ast-stats-1 Block                    288 ( 3.9%)             6            48
ast-stats-1 AssocItem                416 ( 5.6%)             4           104
ast-stats-1 - Type                     208 ( 2.8%)             2
ast-stats-1 - Fn                       208 ( 2.8%)             2
ast-stats-1 GenericParam             480 ( 6.5%)             5            96
ast-stats-1 Expr                     576 ( 7.8%)             8            72
ast-stats-1 - Path                      72 ( 1.0%)             1
ast-stats-1 - Match                     72 ( 1.0%)             1
ast-stats-1 - Struct                    72 ( 1.0%)             1
ast-stats-1 - Lit                      144 ( 1.9%)             2
ast-stats-1 - Block                    216 ( 2.9%)             3
ast-stats-1 Pat                      616 ( 8.3%)             7            88
ast-stats-1 - Struct                    88 ( 1.2%)             1
ast-stats-1 - Wild                      88 ( 1.2%)             1
ast-stats-1 - Ident                    440 ( 5.9%)             5
ast-stats-1 PathSegment              720 ( 9.7%)            30            24
ast-stats-1 Ty                       896 (12.1%)            14            64
ast-stats-1 - Rptr                      64 ( 0.9%)             1
ast-stats-1 - Ptr                       64 ( 0.9%)             1
ast-stats-1 - ImplicitSelf             128 ( 1.7%)             2
ast-stats-1 - Path                     640 ( 8.6%)            10
ast-stats-1 Item                   1_656 (22.3%)             9           184
ast-stats-1 - Trait                    184 ( 2.5%)             1
ast-stats-1 - Enum                     184 ( 2.5%)             1
ast-stats-1 - ForeignMod               184 ( 2.5%)             1
ast-stats-1 - Impl                     184 ( 2.5%)             1
ast-stats-1 - Fn                       368 ( 5.0%)             2
ast-stats-1 - Use                      552 ( 7.4%)             3
ast-stats-1 ----------------------------------------------------------------
ast-stats-1 Total                  7_424
ast-stats-1
ast-stats-2 POST EXPANSION AST STATS
ast-stats-2 Name                Accumulated Size         Count     Item Size
ast-stats-2 ----------------------------------------------------------------
ast-stats-2 ExprField                 48 ( 0.6%)             1            48
ast-stats-2 Crate                     56 ( 0.7%)             1            56
ast-stats-2 GenericArgs               64 ( 0.8%)             1            64
ast-stats-2 - AngleBracketed            64 ( 0.8%)             1
ast-stats-2 Local                     72 ( 0.9%)             1            72
ast-stats-2 WherePredicate            72 ( 0.9%)             1            72
ast-stats-2 - BoundPredicate            72 ( 0.9%)             1
ast-stats-2 Arm                       96 ( 1.2%)             2            48
ast-stats-2 ForeignItem               96 ( 1.2%)             1            96
ast-stats-2 - Fn                        96 ( 1.2%)             1
ast-stats-2 InlineAsm                120 ( 1.5%)             1           120
ast-stats-2 Attribute                128 ( 1.6%)             4            32
ast-stats-2 - DocComment                32 ( 0.4%)             1
ast-stats-2 - Normal                    96 ( 1.2%)             3
ast-stats-2 FieldDef                 160 ( 2.0%)             2            80
ast-stats-2 Stmt                     160 ( 2.0%)             5            32
ast-stats-2 - Local                     32 ( 0.4%)             1
ast-stats-2 - Semi                      32 ( 0.4%)             1
ast-stats-2 - Expr                      96 ( 1.2%)             3
ast-stats-2 Param                    160 ( 2.0%)             4            40
ast-stats-2 FnDecl                   200 ( 2.5%)             5            40
ast-stats-2 Variant                  240 ( 3.0%)             2           120
ast-stats-2 GenericBound             288 ( 3.5%)             4            72
ast-stats-2 - Trait                    288 ( 3.5%)             4
ast-stats-2 Block                    288 ( 3.5%)             6            48
ast-stats-2 AssocItem                416 ( 5.1%)             4           104
ast-stats-2 - Type                     208 ( 2.6%)             2
ast-stats-2 - Fn                       208 ( 2.6%)             2
ast-stats-2 GenericParam             480 ( 5.9%)             5            96
ast-stats-2 Pat                      616 ( 7.6%)             7            88
ast-stats-2 - Struct                    88 ( 1.1%)             1
ast-stats-2 - Wild                      88 ( 1.1%)             1
ast-stats-2 - Ident                    440 ( 5.4%)             5
ast-stats-2 Expr                     648 ( 8.0%)             9            72
ast-stats-2 - Path                      72 ( 0.9%)             1
ast-stats-2 - Match                     72 ( 0.9%)             1
ast-stats-2 - Struct                    72 ( 0.9%)             1
ast-stats-2 - InlineAsm                 72 ( 0.9%)             1
ast-stats-2 - Lit                      144 ( 1.8%)             2
ast-stats-2 - Block                    216 ( 2.7%)             3
ast-stats-2 PathSegment              792 ( 9.8%)            33            24
ast-stats-2 Ty                       896 (11.0%)            14            64
ast-stats-2 - Rptr                      64 ( 0.8%)             1
ast-stats-2 - Ptr                       64 ( 0.8%)             1
ast-stats-2 - ImplicitSelf             128 ( 1.6%)             2
ast-stats-2 - Path                     640 ( 7.9%)            10
ast-stats-2 Item                   2_024 (24.9%)            11           184
ast-stats-2 - Trait                    184 ( 2.3%)             1
ast-stats-2 - Enum                     184 ( 2.3%)             1
ast-stats-2 - ExternCrate              184 ( 2.3%)             1
ast-stats-2 - ForeignMod               184 ( 2.3%)             1
ast-stats-2 - Impl                     184 ( 2.3%)             1
ast-stats-2 - Fn                       368 ( 4.5%)             2
ast-stats-2 - Use                      736 ( 9.1%)             4
ast-stats-2 ----------------------------------------------------------------
ast-stats-2 Total                  8_120
hir-stats HIR STATS
hir-stats HIR STATS
hir-stats Name                Accumulated Size         Count     Item Size
hir-stats ----------------------------------------------------------------
hir-stats ForeignItemRef            24 ( 0.3%)             1            24
hir-stats Lifetime                  32 ( 0.3%)             1            32
hir-stats Mod                       32 ( 0.3%)             1            32
hir-stats ExprField                 40 ( 0.4%)             1            40
hir-stats TraitItemRef              56 ( 0.6%)             2            28
hir-stats Local                     64 ( 0.7%)             1            64
hir-stats Param                     64 ( 0.7%)             2            32
hir-stats InlineAsm                 72 ( 0.8%)             1            72
hir-stats ImplItemRef               72 ( 0.8%)             2            36
hir-stats Body                      96 ( 1.0%)             3            32
hir-stats FieldDef                  96 ( 1.0%)             2            48
hir-stats Arm                       96 ( 1.0%)             2            48
hir-stats Stmt                      96 ( 1.0%)             3            32
hir-stats - Local                     32 ( 0.3%)             1
hir-stats - Semi                      32 ( 0.3%)             1
hir-stats - Expr                      32 ( 0.3%)             1
hir-stats FnDecl                   120 ( 1.3%)             3            40
hir-stats Attribute                128 ( 1.4%)             4            32
hir-stats GenericArg               128 ( 1.4%)             4            32
hir-stats - Type                      32 ( 0.3%)             1
hir-stats - Lifetime                  96 ( 1.0%)             3
hir-stats GenericArgs              144 ( 1.6%)             3            48
hir-stats Variant                  176 ( 1.9%)             2            88
hir-stats GenericBound             192 ( 2.1%)             4            48
hir-stats - Trait                    192 ( 2.1%)             4
hir-stats WherePredicate           192 ( 2.1%)             3            64
hir-stats - BoundPredicate           192 ( 2.1%)             3
hir-stats Block                    288 ( 3.1%)             6            48
hir-stats Pat                      360 ( 3.9%)             5            72
hir-stats - Wild                      72 ( 0.8%)             1
hir-stats - Struct                    72 ( 0.8%)             1
hir-stats - Binding                  216 ( 2.4%)             3
hir-stats GenericParam             400 ( 4.4%)             5            80
hir-stats Generics                 560 ( 6.1%)            10            56
hir-stats Ty                       720 ( 7.8%)            15            48
hir-stats - Ptr                       48 ( 0.5%)             1
hir-stats - Rptr                      48 ( 0.5%)             1
hir-stats - Path                     624 ( 6.8%)            13
hir-stats Expr                     768 ( 8.4%)            12            64
hir-stats - Path                      64 ( 0.7%)             1
hir-stats - Struct                    64 ( 0.7%)             1
hir-stats - Match                     64 ( 0.7%)             1
hir-stats - InlineAsm                 64 ( 0.7%)             1
hir-stats - Lit                      128 ( 1.4%)             2
hir-stats - Block                    384 ( 4.2%)             6
hir-stats Item                     960 (10.5%)            12            80
hir-stats - Trait                     80 ( 0.9%)             1
hir-stats - Enum                      80 ( 0.9%)             1
hir-stats - ExternCrate               80 ( 0.9%)             1
hir-stats - ForeignMod                80 ( 0.9%)             1
hir-stats - Impl                      80 ( 0.9%)             1
hir-stats - Fn                       160 ( 1.7%)             2
hir-stats - Use                      400 ( 4.4%)             5
hir-stats Path                   1_280 (13.9%)            32            40
hir-stats PathSegment            1_920 (20.9%)            40            48
hir-stats ----------------------------------------------------------------
hir-stats
------------------------------------------


