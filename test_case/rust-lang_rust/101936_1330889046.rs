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
Diff in /checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/placeholder_error.rs at line 407:
             let mut self_ty = expected_trait_ref.map(|tr| tr.self_ty());
             self_ty.highlight.maybe_highlighting_region(vid, actual_has_vid);
 
-            if self_ty.value.is_closure()
-                && self.tcx().is_fn_trait(expected_trait_ref.value.def_id)
+            if self_ty.value.is_closure() && self.tcx().is_fn_trait(expected_trait_ref.value.def_id)
             {
                 let closure_sig = self_ty.map(|closure| {
                     if let ty::Closure(_, substs) = closure.kind() {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_infer/src/infer/nll_relate/mod.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/placeholder_error.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/named_anon_conflict.rs" "/checkout/compiler/rustc_infer/src/infer/equate.rs" "/checkout/compiler/rustc_infer/src/errors/mod.rs" "/checkout/compiler/rustc_infer/src/errors/note_and_explain.rs" "/checkout/compiler/rustc_infer/src/lib.rs" "/checkout/compiler/rustc_infer/src/infer/opaque_types.rs"` failed.
Build completed unsuccessfully in 0:00:12
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
