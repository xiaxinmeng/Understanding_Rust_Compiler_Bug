plain
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_mir_build/src/build/mod.rs at line 76:
             kind: hir::TraitItemKind::Const(ty, Some(body_id)),
             ..
         }) => (*body_id, ty.span, None),
-        Node::AnonConst(hir::AnonConst { body, hir_id, .. }) => (*body, tcx.hir().span(*hir_id), None),
+        Node::AnonConst(hir::AnonConst { body, hir_id, .. }) => {
+            (*body, tcx.hir().span(*hir_id), None)
+        }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_build/src/build/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
 
         _ => span_bug!(tcx.hir().span(id), "can't build MIR for {:?}", def.did),
Diff in /checkout/compiler/rustc_mir_build/src/build/mod.rs at line 184:
                 return_ty,
                 return_ty_span,
                 body,
                 body,
-                span_with_body
+                span_with_body,
             );
             mir.yield_ty = yield_ty;
Diff in /checkout/compiler/rustc_mir_build/src/build/mod.rs at line 582:
Diff in /checkout/compiler/rustc_mir_build/src/build/mod.rs at line 582:
     return_ty: Ty<'tcx>,
     return_ty_span: Span,
     body: &'tcx hir::Body<'tcx>,
-    span_with_body: Span
+    span_with_body: Span,
 ) -> Body<'tcx>
 where
     A: Iterator<Item = ArgInfo<'tcx>>,
Diff in /checkout/compiler/rustc_mir_build/src/build/mod.rs at line 658:
     let owner_id = tcx.hir().body_owner(body_id);
     let def_id = tcx.hir().local_def_id(owner_id);
     let span = tcx.hir().span(owner_id);
-    let mut builder = Builder::new(hir, def_id.to_def_id(), span, 0, Safety::Safe, const_ty, const_ty_span, None);
+    let mut builder =
+        Builder::new(hir, def_id.to_def_id(), span, 0, Safety::Safe, const_ty, const_ty_span, None);
     let mut block = START_BLOCK;
     let mut block = START_BLOCK;
     let ast_expr = &tcx.hir().body(body_id).value;
Diff in /checkout/compiler/rustc_mir_build/src/build/mod.rs at line 698:
         hir::BodyOwnerKind::Const => 0,
         hir::BodyOwnerKind::Static(_) => 0,
     };
-    let mut builder = Builder::new(hir, def_id.to_def_id(), span, num_params, Safety::Safe, ty, span, None);
+    let mut builder =
+        Builder::new(hir, def_id.to_def_id(), span, num_params, Safety::Safe, ty, span, None);
     let source_info = builder.source_info(span);
     // Some MIR passes will expect the number of parameters to match the
     // function declaration.
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
