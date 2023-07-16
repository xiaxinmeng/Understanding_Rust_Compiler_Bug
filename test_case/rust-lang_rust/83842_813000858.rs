plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_mir_build/src/thir/cx/block.rs at line 38:
         block_id: hir::ItemLocalId,
         stmts: &'tcx [hir::Stmt<'tcx>],
     ) -> Box<[StmtId]> {
-        stmts.iter().enumerate().filter_map(|(index, stmt)| {
-            let hir_id = stmt.hir_id;
-            let opt_dxn_ext = self.region_scope_tree.opt_destruction_scope(hir_id.local_id);
-            match stmt.kind {
-                hir::StmtKind::Expr(ref expr) | hir::StmtKind::Semi(ref expr) => {
-                    let stmt = Stmt {
-                        kind: StmtKind::Expr {
-                            scope: region::Scope { id: hir_id.local_id, data: region::ScopeData::Node },
-                            expr: self.mirror_expr(expr),
-                        },
-                        opt_destruction_scope: opt_dxn_ext,
-                    };
-                    Some(self.thir.stmts.push(stmt))
-                }
-                hir::StmtKind::Item(..) => {
-                    // ignore for purposes of the MIR
-                }
-                }
-                hir::StmtKind::Local(ref local) => {
-                    let remainder_scope = region::Scope {
-                        id: block_id,
-                        data: region::ScopeData::Remainder(region::FirstStatementIndex::new(index)),
+        stmts
+            .iter()
+            .enumerate()
+            .enumerate()
+            .filter_map(|(index, stmt)| {
+                let hir_id = stmt.hir_id;
+                let opt_dxn_ext = self.region_scope_tree.opt_destruction_scope(hir_id.local_id);
+                match stmt.kind {
+                    hir::StmtKind::Expr(ref expr) | hir::StmtKind::Semi(ref expr) => {
+                        let stmt = Stmt {
+                            kind: StmtKind::Expr {
+                                scope: region::Scope {
+                                    id: hir_id.local_id,
+                                    data: region::ScopeData::Node,
+                                },
+                                expr: self.mirror_expr(expr),
+                            },
+                            opt_destruction_scope: opt_dxn_ext,
+                        };
+                        Some(self.thir.stmts.push(stmt))
+                    }
+                    hir::StmtKind::Item(..) => {
+                        // ignore for purposes of the MIR
+                    }
+                    }
+                    hir::StmtKind::Local(ref local) => {
+                        let remainder_scope = region::Scope {
+                            id: block_id,
+                            data: region::ScopeData::Remainder(region::FirstStatementIndex::new(
+                            )),
+                        };
 
 
-                    let mut pattern = self.pattern_from_hir(local.pat);
+                        let mut pattern = self.pattern_from_hir(local.pat);
 
-                    if let Some(ty) = &local.ty {
-                        if let Some(&user_ty) =
-                            self.typeck_results.user_provided_types().get(ty.hir_id)
-                        {
-                            debug!("mirror_stmts: user_ty={:?}", user_ty);
-                            pattern = Pat {
-                                ty: pattern.ty,
-                                span: pattern.span,
-                                kind: Box::new(PatKind::AscribeUserType {
-                                    ascription: thir::pattern::Ascription {
-                                        user_ty: PatTyProj::from_user_type(user_ty),
-                                        user_ty_span: ty.span,
-                                        variance: ty::Variance::Covariant,
-                                    subpattern: pattern,
-                                }),
-                            };
-                            };
+                        if let Some(ty) = &local.ty {
+                            if let Some(&user_ty) =
+                                self.typeck_results.user_provided_types().get(ty.hir_id)
+                            {
+                                debug!("mirror_stmts: user_ty={:?}", user_ty);
+                                pattern = Pat {
+                                    ty: pattern.ty,
+                                    span: pattern.span,
+                                    kind: Box::new(PatKind::AscribeUserType {
+                                        ascription: thir::pattern::Ascription {
+                                            user_ty: PatTyProj::from_user_type(user_ty),
+                                            user_ty_span: ty.span,
+                                            variance: ty::Variance::Covariant,
+                                        subpattern: pattern,
+                                    }),
+                                };
+                            }
+                            }
                         }
-                    }
 
-                    let stmt = Stmt {
-                        kind: StmtKind::Let {
-                            remainder_scope,
-                            init_scope: region::Scope {
-                                id: hir_id.local_id,
-                                data: region::ScopeData::Node,
+                        let stmt = Stmt {
+                            kind: StmtKind::Let {
+                                remainder_scope,
+                                init_scope: region::Scope {
+                                    id: hir_id.local_id,
+                                    data: region::ScopeData::Node,
+                                pattern,
+                                pattern,
+                                initializer: local.init.map(|init| self.mirror_expr(init)),
+                                lint_level: LintLevel::Explicit(local.hir_id),
-                            pattern,
-                            pattern,
-                            initializer: local.init.map(|init| self.mirror_expr(init)),
-                            lint_level: LintLevel::Explicit(local.hir_id),
-                        },
-                        opt_destruction_scope: opt_dxn_ext,
-                    };
-                    Some(self.thir.stmts.push(stmt))
+                            opt_destruction_scope: opt_dxn_ext,
+                        };
+                        Some(self.thir.stmts.push(stmt))
                 }
-            }
-        }).collect()
+            })
---
Diff in /checkout/compiler/rustc_mir_build/src/build/block.rs at line 96:
                         )
                     );
                 }
-                StmtKind::Let { remainder_scope, init_scope, ref pattern, initializer, lint_level } => {
+                StmtKind::Let {
+                    remainder_scope,
+                    ref pattern,
+                    initializer,
+                    lint_level,
+                } => {
+                } => {
                     let ignores_expr_result = matches!(*pattern.kind, PatKind::Wild);
                     this.block_context.push(BlockFrame::Statement { ignores_expr_result });
 
Diff in /checkout/compiler/rustc_mir_build/src/thir/cx/mod.rs at line 37:
 
 
 impl<'tcx> Cx<'tcx> {
-    fn new(
-        tcx: TyCtxt<'tcx>,
-        def: ty::WithOptConstParam<LocalDefId>,
-    ) -> Cx<'tcx> {
+    fn new(tcx: TyCtxt<'tcx>, def: ty::WithOptConstParam<LocalDefId>) -> Cx<'tcx> {
         let typeck_results = tcx.typeck_opt_const_arg(def);
         Cx {
             tcx,
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/as_rvalue.rs at line 46:
             ExprKind::ThreadLocalRef(did) => block.and(Rvalue::ThreadLocalRef(did)),
             ExprKind::Scope { region_scope, lint_level, value } => {
                 let region_scope = (region_scope, source_info);
-                this.in_scope(region_scope, lint_level, |this| this.as_rvalue(block, scope, &this.thir[value]))
+                this.in_scope(region_scope, lint_level, |this| {
+                    this.as_rvalue(block, scope, &this.thir[value])
             }
             }
             ExprKind::Repeat { value, count } => {
-                let value_operand = unpack!(block = this.as_operand(block, scope, &this.thir[value]));
+                let value_operand =
+                    unpack!(block = this.as_operand(block, scope, &this.thir[value]));
                 block.and(Rvalue::Repeat(value_operand, count))
             }
             ExprKind::Binary { op, lhs, rhs } => {
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/as_rvalue.rs at line 234:
                                     } => unpack!(
                                         block = this.limit_capture_mutability(
                                         block = this.limit_capture_mutability(
-                                            upvar.span, upvar.ty, scope, block, &this.thir[arg],
+                                            upvar.span,
+                                            upvar.ty,
+                                            scope,
+                                            block,
+                                            &this.thir[arg],
                                     ),
                                     ),
                                     _ => unpack!(block = this.as_operand(block, scope, upvar)),
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/as_operand.rs at line 101:
         if let ExprKind::Scope { region_scope, lint_level, value } = expr.kind {
             let source_info = this.source_info(expr.span);
             let region_scope = (region_scope, source_info);
-            return this
-                .in_scope(region_scope, lint_level, |this| this.as_operand(block, scope, &this.thir[value]));
+            return this.in_scope(region_scope, lint_level, |this| {
+                this.as_operand(block, scope, &this.thir[value])
+            });
 
 
         let category = Category::of(&expr.kind).unwrap();
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/as_operand.rs at line 151:
                 // type, and that value is coming from the deref of a box.
                 if let ExprKind::Deref { arg } = expr.kind {
                     // Generate let tmp0 = arg0
-                    let operand = unpack!(block = this.as_temp(block, scope, &this.thir[arg], Mutability::Mut));
+                    let operand = unpack!(
+                        block = this.as_temp(block, scope, &this.thir[arg], Mutability::Mut)
+                    );
 
                     // Return the operand *tmp0 to be used as the call argument
                     let place = Place {
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/as_constant.rs at line 12:
         let this = self;
         let Expr { ty, temp_lifetime: _, span, ref kind } = *expr;
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/as_place.rs at line 444:
         match *kind {
             }
             }
             ExprKind::Field { lhs, name } => {
-                let place_builder =
-                    unpack!(block = this.expr_as_place(block, &this.thir[lhs], mutability, fake_borrow_temps,));
+                let place_builder = unpack!(
+                    block =
+                        this.expr_as_place(block, &this.thir[lhs], mutability, fake_borrow_temps,)
+                );
                 block.and(place_builder.field(name, expr.ty))
             }
             ExprKind::Deref { arg } => {
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/as_place.rs at line 452:
-                let place_builder =
-                    unpack!(block = this.expr_as_place(block, &this.thir[arg], mutability, fake_borrow_temps,));
+                let place_builder = unpack!(
+                    block =
+                        this.expr_as_place(block, &this.thir[arg], mutability, fake_borrow_temps,)
+                );
                 block.and(place_builder.deref())
             }
             ExprKind::Index { lhs, index } => this.lower_index_expression(
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/as_place.rs at line 481:
 
             ExprKind::PlaceTypeAscription { source, user_ty } => {
                 let place_builder = unpack!(
-                    block = this.expr_as_place(block, &this.thir[source], mutability, fake_borrow_temps,)
+                    block = this.expr_as_place(
+                        block,
+                        &this.thir[source],
+                        fake_borrow_temps,
+                    )
                 );
                 );
                 if let Some(user_ty) = user_ty {
                     let annotation_index =
-            ExprKind::Scope { region_scope: _, lint_level: _, value } => this.as_constant(&this.thir[value]),
+            ExprKind::Scope { region_scope: _, lint_level: _, value } => {
+                this.as_constant(&this.thir[value])
+            }
             ExprKind::Literal { literal, user_ty, const_id: _ } => {
                 let user_ty = user_ty.map(|user_ty| {
                     this.canonical_user_type_annotations.push(CanonicalUserTypeAnnotation {
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/stmt.rs at line 95:
                 BreakableTarget::Break(label),
             ),
             ),
-            ExprKind::Return { value } => {
-                this.break_scope(block, value.map(|value| &this.thir[value]), BreakableTarget::Return, source_info)
-            }
+            ExprKind::Return { value } => this.break_scope(
+                block,
+                value.map(|value| &this.thir[value]),
+                BreakableTarget::Return,
+            ),
+            ),
             ExprKind::LlvmInlineAsm { asm, ref outputs, ref inputs } => {
                 debug!("stmt_expr LlvmInlineAsm block_context.push(SubExpr) : {:?}", expr);
                 this.block_context.push(BlockFrame::SubExpr);
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/into.rs at line 54:
             }
             ExprKind::If { cond, then, else_opt } => {
                 let place = unpack!(
-                    block = this.as_temp(block, Some(this.local_scope()), &this.thir[cond], Mutability::Mut)
+                    block = this.as_temp(
+                        block,
+                        Some(this.local_scope()),
+                        &this.thir[cond],
+                        Mutability::Mut
                 );
                 );
                 let operand = Operand::Move(Place::from(place));
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/into.rs at line 63:
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/into.rs at line 63:
                 let term = TerminatorKind::if_(this.tcx, operand, then_block, else_block);
                 this.cfg.terminate(block, source_info, term);
 
-                unpack!(then_block = this.expr_into_dest(destination, then_block, &this.thir[then]));
+                unpack!(
+                    then_block = this.expr_into_dest(destination, then_block, &this.thir[then])
+                );
                 else_block = if let Some(else_opt) = else_opt {
                     unpack!(this.expr_into_dest(destination, else_block, &this.thir[else_opt]))
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/into.rs at line 187:
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/into.rs at line 187:
                     // introduce a unit temporary as the destination for the loop body.
                     let tmp = this.get_unit_temp();
                     // Execute the body, branching back to the test.
-                    let body_block_end = unpack!(this.expr_into_dest(tmp, body_block, &this.thir[body]));
+                    let body_block_end =
+                        unpack!(this.expr_into_dest(tmp, body_block, &this.thir[body]));
                     this.cfg.goto(body_block_end, source_info, loop_block);
 
                     // Loops are only exited by `break` expressions.
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/into.rs at line 268:
                 // (evaluating them in order given by user)
                 let fields_map: FxHashMap<_, _> = fields
                     .into_iter()
-                    .map(|f| (f.name, unpack!(block = this.as_operand(block, Some(scope), &this.thir[f.expr]))))
+                    .map(|f| {
+                            f.name,
+                            unpack!(
+                            unpack!(
+                                block = this.as_operand(block, Some(scope), &this.thir[f.expr])
+                        )
+                    })
                     .collect();
 
 
                 let field_names: Vec<_> =
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/into.rs at line 275:
                     (0..adt_def.variants[variant_index].fields.len()).map(Field::new).collect();
 
                 let fields: Vec<_> = if let Some(FruInfo { base, field_types }) = base {
-                    let place_builder = unpack!(block = this.as_place_builder(block, &this.thir[*base]));
+                    let place_builder =
+                        unpack!(block = this.as_place_builder(block, &this.thir[*base]));
 
                     // MIR does not natively support FRU, so for each
                     // base-supplied field, generate an operand that
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/into.rs at line 334:
                             mir::InlineAsmOperand::Out {
                                 late,
-                                place: expr
-                                place: expr
-                                    .map(|expr| unpack!(block = this.as_place(block, &this.thir[expr]))),
+                                place: expr.map(|expr| {
+                                    unpack!(block = this.as_place(block, &this.thir[expr]))
                             }
                         }
                         }
                         thir::InlineAsmOperand::InOut { reg, late, expr } => {
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/into.rs at line 352:
                             mir::InlineAsmOperand::InOut {
                                 late,
                                 late,
-                                in_value: unpack!(block = this.as_local_operand(block, &this.thir[in_expr])),
+                                in_value: unpack!(
+                                    block = this.as_local_operand(block, &this.thir[in_expr])
+                                ),
                                 out_place: out_expr.map(|out_expr| {
                                     unpack!(block = this.as_place(block, &this.thir[out_expr]))
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/into.rs at line 361:
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/into.rs at line 361:
                         thir::InlineAsmOperand::Const { expr } => mir::InlineAsmOperand::Const {
                             value: unpack!(block = this.as_local_operand(block, &this.thir[expr])),
                         },
-                        thir::InlineAsmOperand::SymFn { expr } => {
-                            mir::InlineAsmOperand::SymFn { value: box this.as_constant(&this.thir[expr]) }
-                        }
+                        thir::InlineAsmOperand::SymFn { expr } => mir::InlineAsmOperand::SymFn {
+                            value: box this.as_constant(&this.thir[expr]),
+                        },
                         thir::InlineAsmOperand::SymStatic { def_id } => {
                             mir::InlineAsmOperand::SymStatic { def_id }
Diff in /checkout/compiler/rustc_mir_build/src/build/mod.rs at line 716:
 ) -> Body<'tcx> {
 ) -> Body<'tcx> {
     let tcx = infcx.tcx;
     let span = tcx.hir().span(hir_id);
-    let mut builder =
-        Builder::new(thir, infcx, def, hir_id, span, 0, Safety::Safe, const_ty, const_ty_span, None);
+    let mut builder = Builder::new(
+        thir,
+        infcx,
+        def,
+        hir_id,
+        span,
+        0,
+        Safety::Safe,
+        const_ty_span,
+        None,
+    );
 
 
     let mut block = START_BLOCK;
     unpack!(block = builder.expr_into_dest(Place::return_place(), block, &thir[expr]));
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_infer/src/infer/outlives/env.rs" "/checkout/compiler/rustc_infer/src/infer/outlives/obligations.rs" "/checkout/compiler/rustc_infer/src/infer/outlives/mod.rs" "/checkout/compiler/rustc_infer/src/infer/fudge.rs" "/checkout/compiler/rustc_mir_build/src/thir/util.rs" "/checkout/compiler/rustc_infer/src/infer/free_regions.rs" "/checkout/compiler/rustc_mir_build/src/thir/cx/mod.rs" "/checkout/compiler/rustc_infer/src/infer/higher_ranked/mod.rs"` failed.
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Diff in /checkout/compiler/rustc_mir_build/src/build/matches/mod.rs at line 158:
Diff in /checkout/compiler/rustc_mir_build/src/build/matches/mod.rs at line 158:
         scrutinee: PlaceBuilder<'tcx>,
         arms: &'pat [ArmId],
     ) -> Vec<(&'pat Arm<'tcx>, Candidate<'pat, 'tcx>)>
-        where 'a: 'pat
+    where
+        'a: 'pat,
     {
         // Assemble a list of candidates: there is one candidate per pattern,
         // which means there may be more than one candidate *per arm*.
