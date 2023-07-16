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
Diff in /checkout/compiler/rustc_lint/src/context.rs at line 144:
         &self.lints
     }
 
-    pub fn get_lint_groups<'t>(&'t self) -> impl Iterator<Item = (&'static str, Vec<LintId>, bool)> + 't {
+    pub fn get_lint_groups<'t>(
+        &'t self,
+    ) -> impl Iterator<Item = (&'static str, Vec<LintId>, bool)> + 't {
         // This function is not used in a way which observes the order of lints.
         #[cfg_attr(not(bootstrap), allow(rustc::potential_query_instability))]
         self.lint_groups
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_passes/src/lib.rs" "/checkout/compiler/rustc_lint/src/traits.rs" "/checkout/compiler/rustc_lint/src/non_ascii_idents.rs" "/checkout/compiler/rustc_passes/src/stability.rs" "/checkout/compiler/rustc_lint/src/types.rs" "/checkout/compiler/rustc_passes/src/entry.rs" "/checkout/compiler/rustc_passes/src/hir_id_validator.rs" "/checkout/compiler/rustc_lint/src/context.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
