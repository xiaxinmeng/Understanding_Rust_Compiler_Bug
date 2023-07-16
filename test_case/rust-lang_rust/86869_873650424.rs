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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_typeck/src/check/upvar.rs at line 635:
 
         // Check whether captured fields also implement the trait
         for capture in root_var_min_capture_list.iter() {
-            let ty = apply_capture_kind_on_capture_ty(self.tcx, capture.place.ty(), capture.info.capture_kind);
+            let ty = apply_capture_kind_on_capture_ty(
+                self.tcx,
+                capture.place.ty(),
+                capture.info.capture_kind,
 
 
             let obligation_holds_for_capture = check_trait
                 .map(|check_trait| self.ty_impls_trait(ty, &cause, check_trait))
Diff in /checkout/compiler/rustc_typeck/src/check/upvar.rs at line 1309:
 
 
 /// Returns a Ty that applies the specified capture kind on the provided capture Ty
-fn apply_capture_kind_on_capture_ty(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>, capture_kind: UpvarCapture<'tcx>) -> Ty<'tcx> {
+fn apply_capture_kind_on_capture_ty(
+    tcx: TyCtxt<'tcx>,
+    ty: Ty<'tcx>,
+    capture_kind: UpvarCapture<'tcx>,
+) -> Ty<'tcx> {
     match capture_kind {
         ty::UpvarCapture::ByValue(_) => ty,
-        ty::UpvarCapture::ByRef(borrow) => tcx.mk_ref(
-            borrow.region,
-            ty::TypeAndMut { ty: ty, mutbl: borrow.kind.to_mutbl_lossy() },
-        ),
+        ty::UpvarCapture::ByRef(borrow) => tcx
+            .mk_ref(borrow.region, ty::TypeAndMut { ty: ty, mutbl: borrow.kind.to_mutbl_lossy() }),
 }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/_match.rs" "/checkout/compiler/rustc_typeck/src/check/op.rs" "/checkout/compiler/rustc_typeck/src/check/pat.rs" "/checkout/compiler/rustc_typeck/src/check/wfcheck.rs" "/checkout/compiler/rustc_typeck/src/check/upvar.rs" "/checkout/compiler/rustc_typeck/src/check/check.rs" "/checkout/compiler/rustc_typeck/src/check/autoderef.rs" "/checkout/compiler/rustc_typeck/src/check/dropck.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
