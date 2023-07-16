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
Diff in /checkout/src/librustdoc/passes/html_tags.rs at line 178:
         let dox = item.attrs.collapsed_doc_value().unwrap_or_default();
         if !dox.is_empty() {
             let report_diag = |msg: &str, range: &Range<usize>| {
-                let sp = match super::source_span_for_markdown_range(tcx, &dox, range, &item.attrs) {
+                let sp = match super::source_span_for_markdown_range(tcx, &dox, range, &item.attrs)
+                {
                     Some(sp) => sp,
                     None => span_of_attrs(&item.attrs).unwrap_or(item.source.span()),
                 };
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/passes/stripper.rs" "/checkout/src/librustdoc/passes/strip_hidden.rs" "/checkout/src/librustdoc/passes/collect_intra_doc_links.rs" "/checkout/src/librustdoc/passes/mod.rs" "/checkout/src/librustdoc/passes/doc_test_lints.rs" "/checkout/src/librustdoc/passes/non_autolinks.rs" "/checkout/src/librustdoc/passes/html_tags.rs" "/checkout/src/librustdoc/passes/strip_priv_imports.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
