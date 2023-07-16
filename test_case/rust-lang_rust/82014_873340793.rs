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
Diff in /checkout/src/librustdoc/passes/mod.rs at line 110:
         ConditionalPass::always(CHECK_INVALID_HTML_TAGS),
         ConditionalPass::always(PROPAGATE_DOC_CFG),
         ConditionalPass::always(CHECK_BARE_URLS),
+    ],
 );
 
 
 /// The list of default passes run when `--doc-coverage` is passed to rustdoc.
Diff in /checkout/src/librustdoc/passes/mod.rs at line 117:
-crate const COVERAGE_PASSES: (&[ConditionalPass], &[ConditionalPass]) = (&[
-    ConditionalPass::new(STRIP_HIDDEN, WhenNotDocumentHidden),
-    ConditionalPass::new(STRIP_PRIVATE, WhenNotDocumentPrivate),
-    ConditionalPass::always(CALCULATE_DOC_COVERAGE),
-], &[]);
+crate const COVERAGE_PASSES: (&[ConditionalPass], &[ConditionalPass]) = (
+    &[
+        ConditionalPass::new(STRIP_HIDDEN, WhenNotDocumentHidden),
+        ConditionalPass::new(STRIP_PRIVATE, WhenNotDocumentPrivate),
+        ConditionalPass::always(CALCULATE_DOC_COVERAGE),
+    &[],
+);
 
 impl ConditionalPass {
 impl ConditionalPass {
     crate const fn always(pass: Pass) -> Self {
Diff in /checkout/src/librustdoc/passes/mod.rs at line 140:
 
 
 /// Returns the given default set of passes.
-crate fn defaults(default_set: DefaultPassOption) -> (&'static [ConditionalPass], &'static [ConditionalPass]) {
+crate fn defaults(
+    default_set: DefaultPassOption,
+) -> (&'static [ConditionalPass], &'static [ConditionalPass]) {
     match default_set {
         DefaultPassOption::Default => DEFAULT_PASSES,
         DefaultPassOption::Coverage => COVERAGE_PASSES,
Diff in /checkout/src/librustdoc/core.rs at line 536:
         krate = run_pass(p, krate, &mut ctxt);
     }
     krate = tcx.sess.time("create_format_cache", || {
-        ctxt.cache.populate(krate, tcx, &ctxt.render_options.extern_html_root_urls, &ctxt.render_options.output)
+        ctxt.cache.populate(
+            krate,
+            tcx,
+            &ctxt.render_options.extern_html_root_urls,
+            &ctxt.render_options.output,
     });
     });
     for p in passes_after_cache {
         krate = run_pass(p, krate, &mut ctxt);
Diff in /checkout/src/librustdoc/passes/collect_intra_doc_links.rs at line 30:
 use std::mem;
 
 
+use crate::clean::{self, utils::find_nearest_parent_module, Crate, Item, ItemLink, PrimitiveType};
 use crate::core::DocContext;
 use crate::fold::DocFolder;
 use crate::html::format::HrefError;
Diff in /checkout/src/librustdoc/passes/collect_intra_doc_links.rs at line 36:
 use crate::html::markdown::{markdown_links, MarkdownLink};
 use crate::lint::{BROKEN_INTRA_DOC_LINKS, PRIVATE_INTRA_DOC_LINKS};
 use crate::passes::Pass;
-use crate::clean::{self, utils::find_nearest_parent_module, Crate, Item, ItemLink, PrimitiveType};
 crate const COLLECT_INTRA_DOC_LINKS: Pass = Pass {
 crate const COLLECT_INTRA_DOC_LINKS: Pass = Pass {
     name: "collect-intra-doc-links",
Diff in /checkout/src/librustdoc/html/format.rs at line 490:
     href_inner(did, cx.cache(), &cx.current)
 
 
-crate fn href_inner(did: DefId, cache: &Cache, relative_to: &[String]) -> Result<(String, ItemType, Vec<String>), HrefError> {
+crate fn href_inner(
+    did: DefId,
+    cache: &Cache,
+    relative_to: &[String],
+) -> Result<(String, ItemType, Vec<String>), HrefError> {
     fn to_module_fqp(shortty: ItemType, fqp: &[String]) -> &[String] {
         if shortty == ItemType::Module { &fqp[..] } else { &fqp[..fqp.len() - 1] }
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_parse/src/lexer/mod.rs" "/checkout/src/librustdoc/passes/strip_hidden.rs" "/checkout/src/librustdoc/markdown.rs" "/checkout/src/librustdoc/passes/mod.rs" "/checkout/src/librustdoc/config.rs" "/checkout/src/librustdoc/passes/stripper.rs" "/checkout/src/librustdoc/doctest.rs" "/checkout/src/librustdoc/lint.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:13
