plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_mir_build/src/thir/constant.rs at line 3:
 use rustc_middle::ty::{self, ParamEnv, ScalarInt, Ty, TyCtxt};
 use rustc_span::DUMMY_SP;
 
-fn trunc<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>, lit: &ast::LitKind, n: u128) -> Result<ScalarInt, LitToConstError> {
+fn trunc<'tcx>(
+    tcx: TyCtxt<'tcx>,
+    ty: Ty<'tcx>,
+    lit: &ast::LitKind,
+    n: u128,
+) -> Result<ScalarInt, LitToConstError> {
     let param_ty = ParamEnv::reveal_all().and(ty);
-    let width = tcx
-        .layout_of(param_ty)
-        .map_err(|_| {
-            LitToConstError::Reported(tcx.sess.delay_span_bug(
-                DUMMY_SP,
-                format!("couldn't compute width of literal: {:?}", lit),
-        })?
-        .size;
+    let width =
+        tcx.layout_of(param_ty)
+        tcx.layout_of(param_ty)
+            .map_err(|_| {
+                LitToConstError::Reported(tcx.sess.delay_span_bug(
+                    DUMMY_SP,
+                    format!("couldn't compute width of literal: {:?}", lit),
+            })?
+            .size;
+            .size;
     trace!("trunc {} with size {} and shift {}", n, width.bits(), 128 - width.bits());
     let result = width.truncate(n);
     trace!("trunc result: {}", result);
Diff in /checkout/compiler/rustc_mir_build/src/thir/constant.rs at line 22:
         .unwrap_or_else(|| bug!("expected to create ScalarInt from uint {:?}", result)))
 
 
-fn get_valtree<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>, neg: bool, lit: &ast::LitKind) -> Result<ty::ValTree<'tcx>, LitToConstError> {
+fn get_valtree<'tcx>(
+    tcx: TyCtxt<'tcx>,
+    neg: bool,
+    neg: bool,
+    lit: &ast::LitKind,
+) -> Result<ty::ValTree<'tcx>, LitToConstError> {
     Ok(match (lit, &ty.kind()) {
         (ast::LitKind::Str(s, _), ty::Ref(_, inner_ty, _)) if inner_ty.is_str() => {
             let str_bytes = s.as_str().as_bytes();
Diff in /checkout/compiler/rustc_mir_build/src/thir/constant.rs at line 42:
             ty::ValTree::from_scalar_int((*n).into())
         }
         (ast::LitKind::Int(n, _), ty::Uint(_)) | (ast::LitKind::Int(n, _), ty::Int(_)) => {
-            let scalar_int =
-                trunc(tcx, ty, lit, if neg { (*n as i128).overflowing_neg().0 as u128 } else { *n })?;
+            let scalar_int = trunc(
+                tcx,
+                ty,
+                lit,
+                if neg { (*n as i128).overflowing_neg().0 as u128 } else { *n },
+            )?;
             ty::ValTree::from_scalar_int(scalar_int)
         }
         (ast::LitKind::Bool(b), ty::Bool) => ty::ValTree::from_scalar_int((*b).into()),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_passes/src/hir_stats.rs" "/checkout/compiler/rustc_passes/src/stability.rs" "/checkout/compiler/rustc_mir_build/src/lib.rs" "/checkout/compiler/rustc_mir_build/src/lints.rs" "/checkout/src/bootstrap/toolstate.rs" "/checkout/compiler/rustc_mir_build/src/thir/constant.rs" "/checkout/compiler/rustc_mir_build/src/thir/util.rs" "/checkout/compiler/rustc_mir_build/src/build/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
