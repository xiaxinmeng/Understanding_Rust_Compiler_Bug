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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_const_eval/src/interpret/place.rs at line 357:
     }
 
     /// Check if this mplace is dereferenceable and sufficiently aligned.
-    pub fn check_mplace(
-        &self,
-        mplace: MPlaceTy<'tcx, M::Provenance>,
-    ) -> InterpResult<'tcx> {
+    pub fn check_mplace(&self, mplace: MPlaceTy<'tcx, M::Provenance>) -> InterpResult<'tcx> {
         let (size, align) = self
             .size_and_align_of_mplace(&mplace)?
             .unwrap_or((mplace.layout.size, mplace.layout.align.abi));
Diff in /checkout/compiler/rustc_const_eval/src/interpret/place.rs at line 367:
         assert!(mplace.align <= align, "dynamic alignment less strict than static one?");
         let align = M::enforce_alignment(self).then_some(align);
-        self.check_ptr_access_align(mplace.ptr, size, align.unwrap_or(Align::ONE), CheckInAllocMsg::DerefTest)?;
+        self.check_ptr_access_align(
+            mplace.ptr,
+            size,
+            align.unwrap_or(Align::ONE),
+            CheckInAllocMsg::DerefTest,
+        )?;
     }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_const_eval/src/interpret/mod.rs" "/checkout/compiler/rustc_const_eval/src/transform/mod.rs" "/checkout/compiler/rustc_const_eval/src/interpret/traits.rs" "/checkout/compiler/rustc_const_eval/src/transform/promote_consts.rs" "/checkout/compiler/rustc_const_eval/src/interpret/operand.rs" "/checkout/compiler/rustc_const_eval/src/transform/validate.rs" "/checkout/compiler/rustc_ast_pretty/src/pp.rs" "/checkout/compiler/rustc_const_eval/src/interpret/place.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
