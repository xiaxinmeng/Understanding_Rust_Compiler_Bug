plain
Successfully built 9f81fcdf47fa
Successfully tagged rust-ci:latest
Built container sha256:9f81fcdf47fa3241769510b073873bfb26099dae41d4fdd6dc9e1a9cec902428
Uploading finished image to https://ci-caches.rust-lang.org/docker/d33e4deadd55ac310bf6d14046967e2ae76df1d0cbcfb3cbdfd57a515110e5c43bea0fc68be40129a2ca62a8c3c0cddee210b82b000006fe1b3d251a4634baf3
upload failed: - to s3://rust-lang-ci-sccache2/docker/d33e4deadd55ac310bf6d14046967e2ae76df1d0cbcfb3cbdfd57a515110e5c43bea0fc68be40129a2ca62a8c3c0cddee210b82b000006fe1b3d251a4634baf3 Unable to locate credentials
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
Highest error code: `E0792`
* 394 features
fmt check
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/src/librustdoc/passes/strip_hidden.rs at line 53:
 
 impl<'a, 'tcx> DocFolder for Stripper<'a, 'tcx> {
     fn fold_item(&mut self, i: Item) -> Option<Item> {
-        let mut is_hidden =
-            self.is_in_hidden_item || i.attrs.lists(sym::doc).has_word(sym::hidden);
+        let mut is_hidden = self.is_in_hidden_item || i.attrs.lists(sym::doc).has_word(sym::hidden);
         if !is_hidden && i.inline_stmt_id.is_none() {
             // We don't need to check if it's coming from a reexport since the reexport itself was
             // already checked.
Diff in /checkout/src/librustdoc/passes/strip_hidden.rs at line 61:
             let hir = self.tcx.hir();
-            is_hidden = i.item_id.as_def_id()
+            is_hidden = i
+                .item_id
+                .as_def_id()
                 .and_then(|def_id| def_id.as_local())
                 .map(|def_id| {
                     let hir_id = hir.local_def_id_to_hir_id(def_id);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/passes/collect_intra_doc_links.rs" "/checkout/src/librustdoc/clean/utils/tests.rs" "/checkout/src/librustdoc/passes/strip_hidden.rs" "/checkout/src/librustdoc/passes/collect_intra_doc_links/early.rs" "/checkout/src/librustdoc/clean/cfg/tests.rs" "/checkout/src/librustdoc/clean/types/tests.rs" "/checkout/src/librustdoc/passes/lint/html_tags.rs" "/checkout/src/bootstrap/doc.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
