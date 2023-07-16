 diff
diff --git a/src/librustc/middle/dead.rs b/src/librustc/middle/dead.rs
index ba810e5..52595d5 100644
--- a/src/librustc/middle/dead.rs
+++ b/src/librustc/middle/dead.rs
@@ -12,16 +12,16 @@
 // closely. The idea is that all reachable symbols are live, codes called
 // from live codes are live, and everything else is dead.

+use middle::lint::{allow, contains_lint, DeadCode};
+use middle::privacy;
 use middle::ty;
 use middle::typeck;
-use middle::privacy;
-use middle::lint::DeadCode;

 use std::hashmap::HashSet;
 use syntax::ast;
 use syntax::ast_map;
 use syntax::ast_util::{local_def, def_id_of_def, is_local};
-use syntax::attr::AttrMetaMethods;
+use syntax::attr;
 use syntax::codemap;
 use syntax::parse::token;
 use syntax::visit::Visitor;
@@ -200,30 +200,22 @@ impl Visitor<()> for MarkSymbolVisitor {
 }

 fn has_allow_dead_code_or_lang_attr(attrs: &[ast::Attribute]) -> bool {
-    for attr in attrs.iter() {
-        if "allow" == attr.name() {
-            if attr.meta_item_list().is_none() {
-                continue
-            }
-            let list = attr.meta_item_list().unwrap();
-            for meta_item in list.iter() {
-                if dead_code_lint_str == meta_item.name() {
-                    return true;
-                }
-            }
-        } else if "lang" == attr.name() {
-            return true;
-        }
-    }
-    false
+    contains_lint(attrs, allow, dead_code_lint_str)
+    || attr::contains_name(attrs, "lang")
 }

 // This visitor seeds items that
-//   1) we want to explicitly consider as live:
+//   1) We want to explicitly consider as live:
 //     * Item annotated with #[allow(dead_code)]
+//         - This is done so that if we want to suppress warnings for a
+//           group of dead functions, we only have to annotate the "root".
+//           For example, if both `f` and `g` are dead and `f` calls `g`,
+//           then annotating `f` with `#[allow(dead_code)]` will suppress
+//           warning for both `f` and `g`.
 //     * Item annotated with #[lang=".."]
+//         - This is because lang items are always callable from elsewhere.
 //   or
-//   2) we are not sure to be live or not
+//   2) We are not sure to be live or not
 //     * Implementation of a trait method
 struct LifeSeeder {
     worklist: ~[ast::NodeId],
diff --git a/src/librustc/middle/lint.rs b/src/librustc/middle/lint.rs
index b468f35..59536c6 100644
--- a/src/librustc/middle/lint.rs
+++ b/src/librustc/middle/lint.rs
@@ -532,6 +532,8 @@ impl<'a> Context<'a> {
     }
 }

+// Check that every lint from the list of attributes satisfies `f`.
+// Return true if that's the case. Otherwise return false.
 pub fn each_lint(sess: session::Session,
                  attrs: &[ast::Attribute],
                  f: |@ast::MetaItem, level, @str| -> bool)
@@ -565,6 +567,25 @@ pub fn each_lint(sess: session::Session,
     true
 }

+// Check from a list of attributes if it contains the appropriate
+// `#[level(lintname)]` attribute (e.g. `#[allow(dead_code)]).
+pub fn contains_lint(attrs: &[ast::Attribute],
+                    level: level, lintname: &'static str) -> bool {
+    let level_name = level_to_str(level);
+    for attr in attrs.iter().filter(|m| level_name == m.name()) {
+        if attr.meta_item_list().is_none() {
+            continue
+        }
+        let list = attr.meta_item_list().unwrap();
+        for meta_item in list.iter() {
+            if lintname == meta_item.name() {
+                return true;
+            }
+        }
+    }
+    false
+}
+
 fn check_while_true_expr(cx: &Context, e: &ast::Expr) {
     match e.node {
         ast::ExprWhile(cond, _) => {
diff --git a/src/test/compile-fail/lint-dead-code-3.rs b/src/test/compile-fail/lint-dead-code-3.rs
index a24b3ae..50c9d50 100644
--- a/src/test/compile-fail/lint-dead-code-3.rs
+++ b/src/test/compile-fail/lint-dead-code-3.rs
@@ -87,6 +87,6 @@ pub fn foo() {
 fn f() { g(); }
 fn g() {}

-// Similarly, lang items are live
-#[lang="fail_"]
+// Similarly, lang items are live (we use a dummy name here)
+#[lang="blah"]
 fn fail(_: *u8, _: *u8, _: uint) -> ! { loop {} }
