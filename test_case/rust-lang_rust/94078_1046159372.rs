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
Diff in /checkout/compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs at line 482:
                         if let Some(ref s) = note {
                             // If it has a custom `#[rustc_on_unimplemented]` note, let's display it
                             if Some(trait_ref.def_id()) == tcx.lang_items().div_trait() {
-                                let span = self.tcx.hir().span(obligation.cause.body_id).with_lo(span.lo());
+                                let span = self
+                                    .tcx
+                                    .hir()
+                                    .span(obligation.cause.body_id)
+                                    .with_lo(span.lo());
                                 if let Ok(call_str) =
                                     self.tcx.sess.source_map().span_to_snippet(span)
                                 {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/query/type_op/normalize.rs" "/checkout/compiler/rustc_trait_selection/src/traits/misc.rs" "/checkout/compiler/rustc_trait_selection/src/traits/chalk_fulfill.rs" "/checkout/compiler/rustc_trait_selection/src/traits/object_safety.rs" "/checkout/compiler/rustc_trait_selection/src/traits/relationships.rs" "/checkout/compiler/rustc_trait_selection/src/traits/specialize/specialization_graph.rs" "/checkout/compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/type_op/implied_outlives_bounds.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
