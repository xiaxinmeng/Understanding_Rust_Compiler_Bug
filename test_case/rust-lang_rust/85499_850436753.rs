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
Diff in /checkout/compiler/rustc_trait_selection/src/traits/query/normalize.rs at line 103:
     where
         T: TypeFoldable<'tcx>,
     {
-        if !self.tcx().sess.opts.debugging_opts.project_under_binders || !t.as_ref().skip_binder().has_escaping_bound_vars() {
+        if !self.tcx().sess.opts.debugging_opts.project_under_binders
+            || !t.as_ref().skip_binder().has_escaping_bound_vars()
+        {
             return t.super_fold_with(self);
         }
         let infcx = self.infcx;
Diff in /checkout/compiler/rustc_trait_selection/src/traits/project.rs at line 330:
     where
         T: TypeFoldable<'tcx>,
     {
-        if !self.tcx().sess.opts.debugging_opts.project_under_binders || !t.as_ref().skip_binder().has_escaping_bound_vars() {
+        if !self.tcx().sess.opts.debugging_opts.project_under_binders
+            || !t.as_ref().skip_binder().has_escaping_bound_vars()
+        {
             return t.super_fold_with(self);
         }
         let infcx = self.selcx.infcx();
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/query/normalize.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/method_autoderef.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/outlives_bounds.rs" "/checkout/compiler/rustc_trait_selection/src/traits/coherence.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/dropck_outlives.rs" "/checkout/compiler/rustc_trait_selection/src/traits/const_evaluatable.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/evaluate_obligation.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:13
