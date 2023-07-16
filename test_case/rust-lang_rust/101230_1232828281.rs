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
Diff in /checkout/compiler/rustc_lint/src/internal.rs at line 397:
         let mut found_impl = false;
         for (hir_id, parent) in cx.tcx.hir().parent_iter(expr.hir_id) {
             if let Some(owner_did) = hir_id.as_owner() {
-                found_parent_with_attr |= cx.tcx.has_attr(
-                    owner_did.to_def_id(), sym::rustc_lint_diagnostics);
+                found_parent_with_attr |=
+                    cx.tcx.has_attr(owner_did.to_def_id(), sym::rustc_lint_diagnostics);
 
 
             debug!(?parent);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_lint/src/traits.rs" "/checkout/compiler/rustc_lint/src/tests.rs" "/checkout/compiler/rustc_lint/src/pass_by_value.rs" "/checkout/compiler/rustc_lint/src/internal.rs" "/checkout/compiler/rustc_macros/src/diagnostics/utils.rs" "/checkout/compiler/rustc_lint/src/expect.rs" "/checkout/compiler/rustc_macros/src/diagnostics/mod.rs" "/checkout/compiler/rustc_lint/src/redundant_semicolon.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
