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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_borrowck/src/member_constraints.rs at line 145:
 where
     R: Copy + Hash + Eq,
 {
-    pub(crate) fn all_indices(&self) -> impl Iterator<Item = NllMemberConstraintIndex> + Captures<'tcx> + '_ {
+    pub(crate) fn all_indices(
+        &self,
+    ) -> impl Iterator<Item = NllMemberConstraintIndex> + Captures<'tcx> + '_ {
         self.constraints.indices()
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_borrowck/src/diagnostics/move_errors.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/mutability_errors.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/find_all_local_uses.rs" "/checkout/compiler/rustc_borrowck/src/constraints/mod.rs" "/checkout/compiler/rustc_borrowck/src/constraints/graph.rs" "/checkout/compiler/rustc_borrowck/src/borrowck_errors.rs" "/checkout/compiler/rustc_borrowck/src/member_constraints.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/outlives_suggestion.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
