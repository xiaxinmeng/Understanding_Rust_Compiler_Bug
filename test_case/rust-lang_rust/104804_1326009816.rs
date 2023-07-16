plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 872631d0f0fadffe3220ab1bd9c8f1f2342341e2 and b35bc7fca50dd5a4b5bcf398ce3fa31b821b2c51
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

Mismatch at src/modules/visitor.rs:84:
 }
 
 impl<'ast> MetaVisitor<'ast> for PathVisitor {
-    fn visit_meta_name_value(&mut self, meta_item: &'ast ast::MetaItem, lit: &'ast ast::MetaItemLit) {
+    fn visit_meta_name_value(
+        &mut self,
+        meta_item: &'ast ast::MetaItem,
+        lit: &'ast ast::MetaItemLit,
+    ) {
         if meta_item.has_name(Symbol::intern("path")) && lit.kind.is_str() {
             self.paths.push(meta_item_lit_to_str(lit));

Mismatch at src/attr.rs:260:
Mismatch at src/attr.rs:260:
     fn rewrite(&self, context: &RewriteContext<'_>, shape: Shape) -> Option<String> {
         match self {
             ast::NestedMetaItem::MetaItem(ref meta_item) => meta_item.rewrite(context, shape),
-            ast::NestedMetaItem::Lit(ref l) => {
-                rewrite_literal(context, l.token_lit, l.span, shape)
-            }
+            ast::NestedMetaItem::Lit(ref l) => rewrite_literal(context, l.token_lit, l.span, shape),
     }
 }

Mismatch at src/attr.rs:527:
Mismatch at src/attr.rs:527:
 
     fn visit_meta_word(&mut self, _meta_item: &'ast ast::MetaItem) {}
 
-    fn visit_meta_name_value(&mut self, _meta_item: &'ast ast::MetaItem, _lit: &'ast ast::MetaItemLit) {}
+    fn visit_meta_name_value(
+        &mut self,
+        _meta_item: &'ast ast::MetaItem,
+        _lit: &'ast ast::MetaItemLit,
+    ) {
 
 
     fn visit_nested_meta_item(&mut self, nm: &'ast ast::NestedMetaItem) {
         match nm {
test test::system_tests ... ok
test test::idempotence_tests ... ok

failures:
failures:

---- test::self_tests stdout ----
Ran 5 self tests.
thread 'test::self_tests' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`: 1 self tests failed', src/tools/rustfmt/src/test/mod.rs:400:5


failures:
    test::self_tests
