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
Diff in /checkout/compiler/rustc_mir/src/borrow_check/diagnostics/conflict_errors.rs at line 1279:
             // to avoid panics
             if !return_ty.has_infer_types() {
                 if let Some(iter_trait) = tcx.get_diagnostic_item(sym::Iterator) {
-                    if tcx.type_implements_trait((iter_trait, return_ty, ty_params, self.param_env)) {
+                    if tcx.type_implements_trait((iter_trait, return_ty, ty_params, self.param_env))
+                    {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir/src/borrow_check/diagnostics/conflict_errors.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
                         err.span_suggestion_short(
                             borrow_span,
                             "Use `.collect()` to allocate the iterator",
Build completed unsuccessfully in 0:00:17
