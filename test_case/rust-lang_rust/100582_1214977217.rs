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
Diff in /checkout/src/librustdoc/json/conversions.rs at line 662:
             Tuple(fields) => Variant::Tuple(
                     .into_iter()
-                    .filter_map(|f| {
-                    .filter_map(|f| {
-                        match *f.kind {
-                            clean::StructFieldItem(ty) => Some(ty.into_tcx(tcx)),
-                            clean::StrippedItem(_) => None,
-                            _ => unreachable!(),
-                        }
+                    .filter_map(|f| match *f.kind {
+                        clean::StructFieldItem(ty) => Some(ty.into_tcx(tcx)),
+                        clean::StrippedItem(_) => None,
+                        _ => unreachable!(),
                     .collect(),
             ),
             ),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/json/conversions.rs" "/checkout/src/librustdoc/passes/collect_intra_doc_links.rs" "/checkout/src/librustdoc/passes/stripper.rs" "/checkout/src/librustdoc/passes/mod.rs" "/checkout/src/librustdoc/lint.rs" "/checkout/src/bootstrap/metrics.rs" "/checkout/src/bootstrap/cache.rs" "/checkout/src/librustdoc/passes/bare_urls.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
