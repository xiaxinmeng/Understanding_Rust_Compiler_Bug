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
Diff in /checkout/compiler/rustc_resolve/src/lib.rs at line 2734:
                         ConstantItemRibKind(trivial, _) => {
                             let features = self.session.features_untracked();
                             // HACK(min_const_generics): We currently only allow `N` or `{ N }`.
-                            if !(trivial
-                                || features.const_generics
-                                || features.generic_const_exprs)
+                            if !(trivial || features.const_generics || features.generic_const_exprs)
                             {
                                 // HACK(min_const_generics): If we encounter `Self` in an anonymous constant
                                 // we can't easily tell if it's generic at this stage, so we instead remember
Diff in /checkout/compiler/rustc_resolve/src/lib.rs at line 2809:
                         ConstantItemRibKind(trivial, _) => {
                             let features = self.session.features_untracked();
                             // HACK(min_const_generics): We currently only allow `N` or `{ N }`.
-                            if !(trivial
-                                || features.const_generics
-                                || features.generic_const_exprs)
+                            if !(trivial || features.const_generics || features.generic_const_exprs)
                             {
                                 if record_used {
                                     self.report_error(
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/query/type_op/custom.rs" "/checkout/compiler/rustc_resolve/src/diagnostics.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/type_op/ascribe_user_type.rs" "/checkout/compiler/rustc_resolve/src/def_collector.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/type_op/eq.rs" "/checkout/compiler/rustc_resolve/src/imports.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/type_op/outlives.rs" "/checkout/compiler/rustc_resolve/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
