plain
Successfully built db460fb7b391
Successfully tagged rust-ci:latest
Built container sha256:db460fb7b3911192434309dcc6df41cd81922597990c80b69cd46922c9f17091
Uploading finished image to https://ci-caches.rust-lang.org/docker/dc5e36fabab9a418dcd1388689e70a41ae6bbea7195d4bd033228b0312de39f9e1d1fdc941775cd0a93a74a0110ae5c291a6c8ed0f0bf9425f4848a8035010bf
upload failed: - to s3://rust-lang-ci-sccache2/docker/dc5e36fabab9a418dcd1388689e70a41ae6bbea7195d4bd033228b0312de39f9e1d1fdc941775cd0a93a74a0110ae5c291a6c8ed0f0bf9425f4848a8035010bf Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
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
Diff in /checkout/compiler/rustc_hir_typeck/src/fn_ctxt/checks.rs at line 765:
                 Compatibility::Incompatible(Some(e)),
             ) = error else { return false };
             let (provided_ty, provided_span) = provided_arg_tys[*provided_idx];
-            let trace = mk_trace(
-                provided_span,
-                formal_and_expected_inputs[*expected_idx],
-                provided_ty,
+            let trace =
+            let trace =
+                mk_trace(provided_span, formal_and_expected_inputs[*expected_idx], provided_ty);
             if !matches!(trace.cause.as_failure_code(*e), FailureCode::Error0308(_)) {
                 self.err_ctxt().report_and_explain_type_error(trace, *e).emit();
                 return true;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_hir_typeck/src/demand.rs" "/checkout/compiler/rustc_hir_typeck/src/fn_ctxt/suggestions.rs" "/checkout/compiler/rustc_hir_typeck/src/fn_ctxt/mod.rs" "/checkout/compiler/rustc_hir_typeck/src/fn_ctxt/checks.rs" "/checkout/compiler/rustc_hir_typeck/src/fn_ctxt/arg_matrix.rs" "/checkout/compiler/rustc_span/src/edition.rs" "/checkout/compiler/rustc_hir_typeck/src/place_op.rs" "/checkout/compiler/rustc_hir_typeck/src/fn_ctxt/_impl.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
