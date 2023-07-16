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
Diff in /checkout/src/librustdoc/passes/collect_intra_doc_links.rs at line 1210:
                     // FIXME: it would be nice to check that the feature gate was enabled in the original crate, not just ignore it altogether.
                     // However I'm not sure how to check that across crates.
                     if prim == PrimitiveType::RawPointer
-                        && item.def_id.is_local() && !self.cx.tcx.features().intra_doc_pointers
+                        && item.def_id.is_local()
+                        && !self.cx.tcx.features().intra_doc_pointers
                     {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/passes/collect_intra_doc_links.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
                         let span = super::source_span_for_markdown_range(
                             cx,
Build completed unsuccessfully in 0:00:28
