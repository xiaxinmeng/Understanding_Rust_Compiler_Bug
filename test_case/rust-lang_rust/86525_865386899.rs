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
Diff in /checkout/compiler/rustc_mir/src/transform/normalize_array_len.rs at line 33:
             local_decls,
             &mut state,
             &mut patches_scratchpad,
-            &mut replacements_scratchpad
+            &mut replacements_scratchpad,
         );
         for el in state.iter_mut() {
             *el = None;
Diff in /checkout/compiler/rustc_mir/src/transform/normalize_array_len.rs at line 52:
 
 
 impl<'a, 'tcx> Patcher<'a, 'tcx> {
-    fn patch_expand_statement(&mut self, statement: &mut Statement<'tcx>) -> Option<std::vec::IntoIter<Statement<'tcx>>> {
+        &mut self,
+        statement: &mut Statement<'tcx>,
+        statement: &mut Statement<'tcx>,
+    ) -> Option<std::vec::IntoIter<Statement<'tcx>>> {
         let idx = self.statement_idx;
         if let Some(len_statemnt_idx) = self.patches_scratchpad.get(&idx).copied() {
             let mut statements = Vec::with_capacity(2);
Diff in /checkout/compiler/rustc_mir/src/transform/normalize_array_len.rs at line 62:
             // use this copy in the Len operation
 
             match &statement.kind {
-                StatementKind::Assign(box(.., Rvalue::Cast(CastKind::Pointer(ty::adjustment::PointerCast::Unsize), operand, _))) => {
+                StatementKind::Assign(box (
+                    ..,
+                    Rvalue::Cast(
+                        CastKind::Pointer(ty::adjustment::PointerCast::Unsize),
+                        _,
+                    ),
+                )) => {
                     match operand {
                     match operand {
                         Operand::Copy(place) | Operand::Move(place) => {
                             // create new local
Diff in /checkout/compiler/rustc_mir/src/transform/normalize_array_len.rs at line 69:
                             let ty = operand.ty(self.local_decls, self.tcx);
-                            let local_decl = LocalDecl::with_source_info(ty, statement.source_info.clone());
+                            let local_decl =
+                                LocalDecl::with_source_info(ty, statement.source_info.clone());
                             let local = self.local_decls.push(local_decl);
                             // make it live
                             let mut make_live_statement = statement.clone();
Diff in /checkout/compiler/rustc_mir/src/transform/normalize_array_len.rs at line 79:
                             let mut make_copy_statement = statement.clone();
                             let assign_to = Place::from(local);
                             let rvalue = Rvalue::Use(operand);
-                            make_copy_statement.kind = StatementKind::Assign(box(assign_to, rvalue));
+                            make_copy_statement.kind =
+                                StatementKind::Assign(box (assign_to, rvalue));
                             statements.push(make_copy_statement);
 
                             // to reorder we have to copy and make NOP
Diff in /checkout/compiler/rustc_mir/src/transform/normalize_array_len.rs at line 87:
                             statement.make_nop();
 
                             self.replacements_scratchpad.insert(len_statemnt_idx, local);
-                        },
-                        _ => {unreachable!("it's a bug in the implementation")}
+                        _ => {
+                        _ => {
+                            unreachable!("it's a bug in the implementation")
                     }
-                },
-                },
-                _ => {unreachable!("it's a bug in the implementation")}
+                _ => {
+                _ => {
+                    unreachable!("it's a bug in the implementation")
             }
 
             self.statement_idx += 1;
             self.statement_idx += 1;
Diff in /checkout/compiler/rustc_mir/src/transform/normalize_array_len.rs at line 101:
             let mut statements = Vec::with_capacity(2);
 
             match &statement.kind {
-                StatementKind::Assign(box(into, Rvalue::Len(place))) => {
+                StatementKind::Assign(box (into, Rvalue::Len(place))) => {
                     let add_deref = if let Some(..) = place.as_local() {
                         false
                     } else if let Some(..) = place.local_or_deref_local() {
Diff in /checkout/compiler/rustc_mir/src/transform/normalize_array_len.rs at line 115:
                     if add_deref {
                         place = self.tcx.mk_place_deref(place);
                     }
-                    len_statement.kind = StatementKind::Assign(box(*into, Rvalue::Len(place)));
+                    len_statement.kind = StatementKind::Assign(box (*into, Rvalue::Len(place)));
                     statements.push(len_statement);
                     // make temporary dead
                     // make temporary dead
Diff in /checkout/compiler/rustc_mir/src/transform/normalize_array_len.rs at line 125:
                     // make original statement NOP
                     // make original statement NOP
                     statement.make_nop();
-                },
-                _ => {unreachable!("it's a bug in the implementation")}
+                _ => {
+                _ => {
+                    unreachable!("it's a bug in the implementation")
             }
 
             self.statement_idx += 1;
             self.statement_idx += 1;
Diff in /checkout/compiler/rustc_mir/src/transform/normalize_array_len.rs at line 145:
     local_decls: &mut IndexVec<Local, LocalDecl<'tcx>>,
     state: &mut IndexVec<Local, Option<usize>>,
     patches_scratchpad: &mut FxIndexMap<usize, usize>,
-    replacements_scratchpad: &mut FxIndexMap<usize, Local>
+    replacements_scratchpad: &mut FxIndexMap<usize, Local>,
 ) {
     for (statement_idx, statement) in block.statements.iter_mut().enumerate() {
         match &mut statement.kind {
Diff in /checkout/compiler/rustc_mir/src/transform/normalize_array_len.rs at line 152:
             StatementKind::Assign(box (place, rvalue)) => {
                 match rvalue {
-                    Rvalue::Cast(CastKind::Pointer(ty::adjustment::PointerCast::Unsize), operand, cast_ty) => {
-                        let local = if let Some(local) = place.as_local() {local} else {return};
+                    Rvalue::Cast(
+                        CastKind::Pointer(ty::adjustment::PointerCast::Unsize),
+                        cast_ty,
+                    ) => {
+                    ) => {
+                        let local = if let Some(local) = place.as_local() { local } else { return };
                         match operand {
                             Operand::Copy(place) | Operand::Move(place) => {
-                                let operand_local = if let Some(local) = place.local_or_deref_local() {
-                                } else {
-                                    return
-                                };
+                                let operand_local =
+                                let operand_local =
+                                    if let Some(local) = place.local_or_deref_local() {
+                                    } else {
+                                        return;
+                                    };
+                                    };
                                 let operand_ty = local_decls[operand_local].ty;
                                 match (operand_ty.kind(), cast_ty.kind()) {
                                     (ty::Array(of_ty_src, ..), ty::Slice(of_ty_dst)) => {
Diff in /checkout/compiler/rustc_mir/src/transform/normalize_array_len.rs at line 167:
                                             // this is a cast from [T; N] into [T], so we are good
                                             state[local] = Some(statement_idx);
-                                    },
+                                    }
+                                    }
                                     // current way of patching doesn't allow to work with `mut`
-                                    (ty::Ref(ty::RegionKind::ReErased, operand_ty, Mutability::Not), ty::Ref(ty::RegionKind::ReErased, cast_ty, Mutability::Not)) => {
+                                        ty::Ref(
+                                        ty::Ref(
+                                            ty::RegionKind::ReErased,
+                                            operand_ty,
+                                        ),
+                                        ),
+                                        ty::Ref(ty::RegionKind::ReErased, cast_ty, Mutability::Not),
+                                    ) => {
                                         match (operand_ty.kind(), cast_ty.kind()) {
                                             // current way of patching doesn't allow to work with `mut`
                                             (ty::Array(of_ty_src, ..), ty::Slice(of_ty_dst)) => {
Diff in /checkout/compiler/rustc_mir/src/transform/normalize_array_len.rs at line 177:
                                                     // this is a cast from [T; N] into [T], so we are good
                                                     state[local] = Some(statement_idx);
-                                            },
+                                            }
                                             _ => {}
                                         }
---
                             _ => {}
                         }
-                    },
+                    }
                     Rvalue::Len(place) => {
                         let local = if let Some(local) = place.local_or_deref_local() {
                             local
Diff in /checkout/compiler/rustc_mir/src/transform/normalize_array_len.rs at line 193:
-                            return
+                            return;
                         };
                         };
                         if let Some(cast_statement_idx) = state[local] {
                             patches_scratchpad.insert(cast_statement_idx, statement_idx);
Diff in /checkout/compiler/rustc_mir/src/transform/normalize_array_len.rs at line 198:
-                    },
+                    }
                     _ => {
                         // invalidate
                         // invalidate
                         state[place.local] = None;
Diff in /checkout/compiler/rustc_mir/src/transform/normalize_array_len.rs at line 203:
+                    }
                 }
-            },
+            }
+            }
             _ => {}
         }
     }
Diff in /checkout/compiler/rustc_mir/src/transform/normalize_array_len.rs at line 215:
         statement_idx: 0,
 
 
-    block.expand_statements(|st| {
-        patcher.patch_expand_statement(st)
-    });
+    block.expand_statements(|st| patcher.patch_expand_statement(st));
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir/src/transform/add_retag.rs" "/checkout/compiler/rustc_mir/src/transform/early_otherwise_branch.rs" "/checkout/compiler/rustc_mir/src/transform/instcombine.rs" "/checkout/compiler/rustc_mir/src/transform/promote_consts.rs" "/checkout/compiler/rustc_mir/src/transform/simplify_try.rs" "/checkout/compiler/rustc_mir/src/transform/validate.rs" "/checkout/compiler/rustc_mir/src/transform/normalize_array_len.rs" "/checkout/compiler/rustc_mir/src/transform/required_consts.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:11
