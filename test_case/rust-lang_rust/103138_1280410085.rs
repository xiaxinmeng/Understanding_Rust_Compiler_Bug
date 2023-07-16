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
Diff in /checkout/compiler/rustc_codegen_ssa/src/mir/block.rs at line 131:
         fx: &mut FunctionCx<'a, 'tcx, Bx>,
         bx: &mut Bx,
-        follow_on: bool
+        follow_on: bool,
     ) -> bool {
     ) -> bool {
         // njn: duplicated stuff from .lltarget()
         // njn: also, some non-(None,None) cases in .lltarget() that could be used here
Diff in /checkout/compiler/rustc_codegen_ssa/src/mir/block.rs at line 834:
         if intrinsic == Some(sym::caller_location) {
             if let Some(target) = target {
-                let location = self
-                let location = self
-                    .get_caller_location(bx, mir::SourceInfo { span: fn_span, ..source_info });
+                let location =
+                    self.get_caller_location(bx, mir::SourceInfo { span: fn_span, ..source_info });
 
                 if let ReturnDest::IndirectOperand(tmp, _) = ret_dest {
                     location.val.store(bx, tmp);
Diff in /checkout/compiler/rustc_codegen_ssa/src/mir/block.rs at line 1019:
             self.codegen_argument(bx, op, &mut llargs, &fn_abi.args[i]);
         }
         let num_untupled = untuple.map(|tup| {
-            self.codegen_arguments_untupled(
-                bx,
-                tup,
-                &mut llargs,
-                &fn_abi.args[first_args.len()..],
-            )
+            self.codegen_arguments_untupled(bx, tup, &mut llargs, &fn_abi.args[first_args.len()..])
 
         let needs_location =
Diff in /checkout/compiler/rustc_codegen_ssa/src/mir/block.rs at line 1330:
                     target,
                     target,
                     cleanup,
                     fn_span,
-                    follow_on
+                    follow_on,
                 )
             }
             mir::TerminatorKind::GeneratorDrop | mir::TerminatorKind::Yield { .. } => {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_hir_analysis/src/check/fn_ctxt/mod.rs" "/checkout/compiler/rustc_hir_analysis/src/check/fn_ctxt/suggestions.rs" "/checkout/compiler/rustc_hir_analysis/src/check/fn_ctxt/_impl.rs" "/checkout/compiler/rustc_hir_analysis/src/check/fn_ctxt/arg_matrix.rs" "/checkout/compiler/rustc_hir_analysis/src/check/fn_ctxt/checks.rs" "/checkout/compiler/rustc_hir_analysis/src/check/method/prelude2021.rs" "/checkout/compiler/rustc_codegen_ssa/src/mir/block.rs" "/checkout/compiler/rustc_codegen_ssa/src/base.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
