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
Diff in /checkout/compiler/rustc_const_eval/src/interpret/eval_context.rs at line 929:
     }
 
     #[must_use]
-    pub fn generate_stacktrace_from_stack(stack: &[Frame<'mir, 'tcx, M::Provenance, M::FrameExtra>]) -> Vec<FrameInfo<'tcx>> {
+    pub fn generate_stacktrace_from_stack(
+        stack: &[Frame<'mir, 'tcx, M::Provenance, M::FrameExtra>],
         let mut frames = Vec::new();
         let mut frames = Vec::new();
         // This deliberately does *not* honor `requires_caller_location` since it is used for much
         // more than just panics.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/query/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/normalize.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/dropck_outlives.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/method_autoderef.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/evaluate_obligation.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/type_op/normalize.rs" "/checkout/compiler/rustc_trait_selection/src/traits/select/mod.rs" "/checkout/compiler/rustc_const_eval/src/interpret/eval_context.rs"` failed.
Build completed unsuccessfully in 0:00:11
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
