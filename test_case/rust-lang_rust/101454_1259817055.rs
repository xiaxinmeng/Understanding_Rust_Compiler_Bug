plain

---- [ui] src/test/ui/stats/hir-stats.rs stdout ----
diff of stderr:

140 hir-stats FnDecl                   120 ( 1.2%)             3            40
141 hir-stats Attribute                128 ( 1.3%)             4            32
142 hir-stats GenericArgs              144 ( 1.5%)             3            48
- hir-stats Variant                  160 ( 1.7%)             2            80
- hir-stats WherePredicate           168 ( 1.7%)             3            56
- hir-stats - BoundPredicate           168 ( 1.7%)             3
+ hir-stats Variant                  160 ( 1.6%)             2            80
146 hir-stats GenericBound             192 ( 2.0%)             4            48
147 hir-stats - Trait                    192 ( 2.0%)             4
+ hir-stats WherePredicate           192 ( 2.0%)             3            64
+ hir-stats - BoundPredicate           192 ( 2.0%)             3
148 hir-stats Block                    288 ( 3.0%)             6            48
149 hir-stats Pat                      360 ( 3.7%)             5            72
150 hir-stats - Wild                      72 ( 0.7%)             1

169 hir-stats - ExternCrate               80 ( 0.8%)             1
170 hir-stats - ForeignMod                80 ( 0.8%)             1
171 hir-stats - Impl                      80 ( 0.8%)             1
- hir-stats - Fn                       160 ( 1.7%)             2
+ hir-stats - Fn                       160 ( 1.6%)             2
173 hir-stats - Use                      400 ( 4.1%)             5
- hir-stats Path                   1_536 (15.9%)            32            48
+ hir-stats Path                   1_536 (15.8%)            32            48
175 hir-stats PathSegment            2_240 (23.1%)            40            56
176 hir-stats ----------------------------------------------------------------
- hir-stats Total                  9_680
+ hir-stats Total                  9_704
179 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stats/hir-stats/hir-stats.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args stats/hir-stats.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/stats/hir-stats.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stats/hir-stats" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zhir-stats" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stats/hir-stats/auxiliary"
stdout: none
--- stderr -------------------------------
ast-stats-1 PRE EXPANSION AST STATS
ast-stats-1 Name                Accumulated Size         Count     Item Size
ast-stats-1 ----------------------------------------------------------------
ast-stats-1 ExprField                 48 ( 0.6%)             1            48
ast-stats-1 Crate                     56 ( 0.7%)             1            56
ast-stats-1 Attribute                 64 ( 0.8%)             2            32
ast-stats-1 - Normal                    32 ( 0.4%)             1
ast-stats-1 - DocComment                32 ( 0.4%)             1
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
ast-stats-1 GenericArgs               64 ( 0.8%)             1            64
ast-stats-1 - AngleBracketed            64 ( 0.8%)             1
ast-stats-1 Local                     72 ( 0.9%)             1            72
ast-stats-1 WherePredicate            72 ( 0.9%)             1            72
ast-stats-1 - BoundPredicate            72 ( 0.9%)             1
ast-stats-1 Arm                       96 ( 1.1%)             2            48
ast-stats-1 ForeignItem               96 ( 1.1%)             1            96
ast-stats-1 - Fn                        96 ( 1.1%)             1
ast-stats-1 FieldDef                 160 ( 1.9%)             2            80
ast-stats-1 Stmt                     160 ( 1.9%)             5            32
ast-stats-1 - Local                     32 ( 0.4%)             1
ast-stats-1 - MacCall                   32 ( 0.4%)             1
ast-stats-1 - Expr                      96 ( 1.1%)             3
ast-stats-1 Param                    160 ( 1.9%)             4            40
ast-stats-1 FnDecl                   200 ( 2.4%)             5            40
ast-stats-1 Variant                  240 ( 2.9%)             2           120
ast-stats-1 Block                    288 ( 3.4%)             6            48
ast-stats-1 GenericBound             352 ( 4.2%)             4            88
ast-stats-1 - Trait                    352 ( 4.2%)             4
ast-stats-1 AssocItem                416 ( 4.9%)             4           104
ast-stats-1 - TyAlias                  208 ( 2.5%)             2
ast-stats-1 - Fn                       208 ( 2.5%)             2
ast-stats-1 GenericParam             480 ( 5.7%)             5            96
ast-stats-1 PathSegment              720 ( 8.6%)            30            24
ast-stats-1 Expr                     832 ( 9.9%)             8           104
ast-stats-1 - Path                     104 ( 1.2%)             1
ast-stats-1 - Match                    104 ( 1.2%)             1
ast-stats-1 - Struct                   104 ( 1.2%)             1
ast-stats-1 - Lit                      208 ( 2.5%)             2
ast-stats-1 - Block                    312 ( 3.7%)             3
ast-stats-1 Pat                      840 (10.0%)             7           120
ast-stats-1 - Struct                   120 ( 1.4%)             1
ast-stats-1 - Wild                     120 ( 1.4%)             1
ast-stats-1 - Ident                    600 ( 7.1%)             5
ast-stats-1 Ty                     1_344 (16.0%)            14            96
ast-stats-1 - Rptr                      96 ( 1.1%)             1
ast-stats-1 - Ptr                       96 ( 1.1%)             1
ast-stats-1 - ImplicitSelf             192 ( 2.3%)             2
ast-stats-1 - Path                     960 (11.4%)            10
ast-stats-1 Item                   1_656 (19.7%)             9           184
ast-stats-1 - Trait                    184 ( 2.2%)             1
ast-stats-1 - Enum                     184 ( 2.2%)             1
ast-stats-1 - ForeignMod               184 ( 2.2%)             1
ast-stats-1 - Impl                     184 ( 2.2%)             1
ast-stats-1 - Fn                       368 ( 4.4%)             2
ast-stats-1 - Use                      552 ( 6.6%)             3
ast-stats-1 ----------------------------------------------------------------
ast-stats-1 Total                  8_416
ast-stats-1
ast-stats-2 POST EXPANSION AST STATS
ast-stats-2 Name                Accumulated Size         Count     Item Size
ast-stats-2 ----------------------------------------------------------------
ast-stats-2 ExprField                 48 ( 0.5%)             1            48
ast-stats-2 Crate                     56 ( 0.6%)             1            56
ast-stats-2 GenericArgs               64 ( 0.7%)             1            64
ast-stats-2 - AngleBracketed            64 ( 0.7%)             1
ast-stats-2 Local                     72 ( 0.8%)             1            72
ast-stats-2 WherePredicate            72 ( 0.8%)             1            72
ast-stats-2 - BoundPredicate            72 ( 0.8%)             1
ast-stats-2 Arm                       96 ( 1.0%)             2            48
ast-stats-2 ForeignItem               96 ( 1.0%)             1            96
ast-stats-2 - Fn                        96 ( 1.0%)             1
ast-stats-2 InlineAsm                120 ( 1.3%)             1           120
ast-stats-2 Attribute                128 ( 1.4%)             4            32
ast-stats-2 - DocComment                32 ( 0.3%)             1
ast-stats-2 - Normal                    96 ( 1.0%)             3
ast-stats-2 FieldDef                 160 ( 1.7%)             2            80
ast-stats-2 Stmt                     160 ( 1.7%)             5            32
ast-stats-2 - Local                     32 ( 0.3%)             1
ast-stats-2 - Semi                      32 ( 0.3%)             1
ast-stats-2 - Expr                      96 ( 1.0%)             3
ast-stats-2 Param                    160 ( 1.7%)             4            40
ast-stats-2 FnDecl                   200 ( 2.2%)             5            40
ast-stats-2 Variant                  240 ( 2.6%)             2           120
ast-stats-2 Block                    288 ( 3.1%)             6            48
ast-stats-2 GenericBound             352 ( 3.8%)             4            88
ast-stats-2 - Trait                    352 ( 3.8%)             4
ast-stats-2 AssocItem                416 ( 4.5%)             4           104
ast-stats-2 - TyAlias                  208 ( 2.3%)             2
ast-stats-2 - Fn                       208 ( 2.3%)             2
ast-stats-2 GenericParam             480 ( 5.2%)             5            96
ast-stats-2 PathSegment              792 ( 8.7%)            33            24
ast-stats-2 Pat                      840 ( 9.2%)             7           120
ast-stats-2 - Struct                   120 ( 1.3%)             1
ast-stats-2 - Wild                     120 ( 1.3%)             1
ast-stats-2 - Ident                    600 ( 6.6%)             5
ast-stats-2 Expr                     936 (10.2%)             9           104
ast-stats-2 - Path                     104 ( 1.1%)             1
ast-stats-2 - Match                    104 ( 1.1%)             1
ast-stats-2 - Struct                   104 ( 1.1%)             1
ast-stats-2 - InlineAsm                104 ( 1.1%)             1
ast-stats-2 - Lit                      208 ( 2.3%)             2
ast-stats-2 - Block                    312 ( 3.4%)             3
ast-stats-2 Ty                     1_344 (14.7%)            14            96
ast-stats-2 - Rptr                      96 ( 1.0%)             1
ast-stats-2 - Ptr                       96 ( 1.0%)             1
ast-stats-2 - ImplicitSelf             192 ( 2.1%)             2
ast-stats-2 - Path                     960 (10.5%)            10
ast-stats-2 Item                   2_024 (22.1%)            11           184
ast-stats-2 - Trait                    184 ( 2.0%)             1
ast-stats-2 - Enum                     184 ( 2.0%)             1
ast-stats-2 - ExternCrate              184 ( 2.0%)             1
ast-stats-2 - ForeignMod               184 ( 2.0%)             1
ast-stats-2 - Impl                     184 ( 2.0%)             1
ast-stats-2 - Fn                       368 ( 4.0%)             2
ast-stats-2 - Use                      736 ( 8.0%)             4
ast-stats-2 ----------------------------------------------------------------
ast-stats-2 Total                  9_144
hir-stats HIR STATS
hir-stats HIR STATS
hir-stats Name                Accumulated Size         Count     Item Size
hir-stats ----------------------------------------------------------------
hir-stats ForeignItemRef            24 ( 0.2%)             1            24
hir-stats Lifetime                  32 ( 0.3%)             1            32
hir-stats Mod                       32 ( 0.3%)             1            32
hir-stats ExprField                 40 ( 0.4%)             1            40
hir-stats TraitItemRef              56 ( 0.6%)             2            28
hir-stats Local                     64 ( 0.7%)             1            64
hir-stats Param                     64 ( 0.7%)             2            32
hir-stats InlineAsm                 72 ( 0.7%)             1            72
hir-stats ImplItemRef               72 ( 0.7%)             2            36
hir-stats Body                      96 ( 1.0%)             3            32
hir-stats GenericArg                96 ( 1.0%)             4            24
hir-stats - Type                      24 ( 0.2%)             1
hir-stats - Lifetime                  72 ( 0.7%)             3
hir-stats FieldDef                  96 ( 1.0%)             2            48
hir-stats Arm                       96 ( 1.0%)             2            48
hir-stats Stmt                      96 ( 1.0%)             3            32
hir-stats - Local                     32 ( 0.3%)             1
hir-stats - Semi                      32 ( 0.3%)             1
hir-stats - Expr                      32 ( 0.3%)             1
hir-stats FnDecl                   120 ( 1.2%)             3            40
hir-stats Attribute                128 ( 1.3%)             4            32
hir-stats GenericArgs              144 ( 1.5%)             3            48
hir-stats Variant                  160 ( 1.6%)             2            80
hir-stats GenericBound             192 ( 2.0%)             4            48
hir-stats - Trait                    192 ( 2.0%)             4
hir-stats WherePredicate           192 ( 2.0%)             3            64
hir-stats - BoundPredicate           192 ( 2.0%)             3
hir-stats Block                    288 ( 3.0%)             6            48
hir-stats Pat                      360 ( 3.7%)             5            72
hir-stats - Wild                      72 ( 0.7%)             1
hir-stats - Struct                    72 ( 0.7%)             1
hir-stats - Binding                  216 ( 2.2%)             3
hir-stats GenericParam             400 ( 4.1%)             5            80
hir-stats Generics                 560 ( 5.8%)            10            56
hir-stats Ty                       720 ( 7.4%)            15            48
hir-stats - Ptr                       48 ( 0.5%)             1
hir-stats - Rptr                      48 ( 0.5%)             1
hir-stats - Path                     624 ( 6.4%)            13
hir-stats Expr                     768 ( 7.9%)            12            64
hir-stats - Path                      64 ( 0.7%)             1
hir-stats - Struct                    64 ( 0.7%)             1
hir-stats - Match                     64 ( 0.7%)             1
hir-stats - InlineAsm                 64 ( 0.7%)             1
hir-stats - Lit                      128 ( 1.3%)             2
hir-stats - Block                    384 ( 4.0%)             6
hir-stats Item                     960 ( 9.9%)            12            80
hir-stats - Trait                     80 ( 0.8%)             1
hir-stats - Enum                      80 ( 0.8%)             1
hir-stats - ExternCrate               80 ( 0.8%)             1
hir-stats - ForeignMod                80 ( 0.8%)             1
hir-stats - Impl                      80 ( 0.8%)             1
hir-stats - Fn                       160 ( 1.6%)             2
hir-stats - Use                      400 ( 4.1%)             5
hir-stats Path                   1_536 (15.8%)            32            48
hir-stats PathSegment            2_240 (23.1%)            40            56
hir-stats ----------------------------------------------------------------
hir-stats Total                  9_704
------------------------------------------



