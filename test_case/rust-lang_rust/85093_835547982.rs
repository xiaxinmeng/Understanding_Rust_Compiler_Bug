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
Diff in /checkout/src/librustdoc/passes/calculate_doc_coverage.rs at line 215:
                 let has_doc_example = tests.found_tests != 0;
                 // The `expect_real()` should be okay because `local_def_id_to_hir_id`
                 // would presumably panic if a fake `DefIndex` were passed.
-                let hir_id = self.ctx.tcx.hir().local_def_id_to_hir_id(i.def_id.expect_real().expect_local());
+                let hir_id = self
+                    .ctx
+                    .tcx
+                    .hir()
+                    .local_def_id_to_hir_id(i.def_id.expect_real().expect_local());
                 let (level, source) = self.ctx.tcx.lint_level_at_node(MISSING_DOCS, hir_id);
                 // `missing_docs` is allow-by-default, so don't treat this as ignoring the item
                 // unless the user had an explicit `allow`
Diff in /checkout/src/librustdoc/passes/collect_intra_doc_links.rs at line 1209:
                 .as_local()
                 // The `expect_real()` should be okay because `local_def_id_to_hir_id`
                 // would presumably panic if a fake `DefIndex` were passed.
-                .and_then(|dst_id| item.def_id.expect_real().as_local().map(|src_id| (src_id, dst_id)))
+                .and_then(|dst_id| {
+                    item.def_id.expect_real().as_local().map(|src_id| (src_id, dst_id))
             {
                 use rustc_hir::def_id::LOCAL_CRATE;
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/passes/bare_urls.rs" "/checkout/src/bootstrap/bin/rustdoc.rs" "/checkout/src/librustdoc/passes/strip_hidden.rs" "/checkout/src/bootstrap/bin/llvm-config-wrapper.rs" "/checkout/src/bootstrap/bin/rustc.rs" "/checkout/src/bootstrap/format.rs" "/checkout/src/librustdoc/passes/html_tags.rs" "/checkout/src/librustdoc/passes/calculate_doc_coverage.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:13
