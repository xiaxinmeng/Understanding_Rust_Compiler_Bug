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
Diff in /checkout/compiler/rustc_middle/src/middle/privacy.rs at line 180:
             .copied()
             .unwrap_or_else(|| EffectiveVisibility::from_vis(default_vis()));
         if let Some(inherited_effective_vis) = self.get_effective_vis(parent_id) {
-            let mut inherited_effective_vis_at_prev_level = inherited_effective_vis.get(tag).clone();
+            let mut inherited_effective_vis_at_prev_level =
+                inherited_effective_vis.get(tag).clone();
             for level in AccessLevel::all_levels() {
                 if tag >= level {
-                    let inherited_effective_vis_at_level = inherited_effective_vis.get(level).clone();
-                    if !(inherited_effective_vis_at_prev_level == inherited_effective_vis_at_level && tag != level) {
+                    let inherited_effective_vis_at_level =
+                        inherited_effective_vis.get(level).clone();
+                    if !(inherited_effective_vis_at_prev_level == inherited_effective_vis_at_level
+                        && tag != level)
                         let calculated_effective_vis =
                         let calculated_effective_vis =
-                        if nominal_vis.is_at_least(inherited_effective_vis_at_level, tree) {
-                            inherited_effective_vis_at_level
-                            nominal_vis
-                        };
-                        };
-                        changed |= current_effective_vis.update(calculated_effective_vis, level, tree);
+                            if nominal_vis.is_at_least(inherited_effective_vis_at_level, tree) {
+                                inherited_effective_vis_at_level
+                                nominal_vis
+                            };
+                        changed |=
+                        changed |=
+                            current_effective_vis.update(calculated_effective_vis, level, tree);
                     }
                     inherited_effective_vis_at_prev_level = inherited_effective_vis_at_level;
                 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/middle/lang_items.rs" "/checkout/compiler/rustc_middle/src/middle/resolve_lifetime.rs" "/checkout/compiler/rustc_middle/src/middle/mod.rs" "/checkout/compiler/rustc_middle/src/middle/privacy.rs" "/checkout/compiler/rustc_middle/src/middle/stability.rs" "/checkout/compiler/rustc_middle/src/middle/limits.rs" "/checkout/compiler/rustc_middle/src/middle/dependency_format.rs" "/checkout/compiler/rustc_middle/src/middle/exported_symbols.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
