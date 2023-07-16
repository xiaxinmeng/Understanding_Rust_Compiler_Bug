plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
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
Diff in /checkout/compiler/rustc_typeck/src/check/generator_interior/drop_ranges/record_consumed_borrow.rs at line 53:
     }
 
-    fn consume_body(
-        &mut self,
-        &mut self,
-        fcx: &'_ FnCtxt<'_, 'tcx>,
-        def_id: DefId,
-        body: &'tcx Body<'tcx>,
-    ) {
+    fn consume_body(&mut self, fcx: &'_ FnCtxt<'_, 'tcx>, def_id: DefId, body: &'tcx Body<'tcx>) {
         // Run ExprUseVisitor to find where values are consumed.
         ExprUseVisitor::new(
             self,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/closure.rs" "/checkout/compiler/rustc_typeck/src/check/generator_interior/drop_ranges/cfg_propagate.rs" "/checkout/compiler/rustc_typeck/src/check/fallback.rs" "/checkout/compiler/rustc_typeck/src/check/generator_interior/drop_ranges/cfg_visualize.rs" "/checkout/compiler/rustc_typeck/src/check/intrinsic.rs" "/checkout/compiler/rustc_typeck/src/check/wfcheck.rs" "/checkout/compiler/rustc_typeck/src/check/generator_interior/drop_ranges/cfg_build.rs" "/checkout/compiler/rustc_typeck/src/check/generator_interior/drop_ranges/record_consumed_borrow.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
