plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between f241c0c43d71960f078b897e9b8721d4b452ce5e and 82b3225a462bb5ae037563354f98ba9a592effbc
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

test test::verify_config_test_names ... ok
test test::configuration_snippet::configuration_snippet_tests ... ok

Mismatch at src/patterns.rs:46:
         | ast::PatKind::Path(..)
         | ast::PatKind::Range(..) => false,
         ast::PatKind::Tuple(ref subpats) => subpats.len() <= 1,
-        ast::PatKind::TupleStruct(ref ts) => {
-            ts.path.segments.len() <= 1 && ts.pats.len() <= 1
-        }
+        ast::PatKind::TupleStruct(ref ts) => ts.path.segments.len() <= 1 && ts.pats.len() <= 1,
         ast::PatKind::Box(ref p) | ast::PatKind::Ref(ref p, _) | ast::PatKind::Paren(ref p) => {
             is_short_pattern_inner(&*p)

Mismatch at src/patterns.rs:228:
Mismatch at src/patterns.rs:228:
                 rewrite_path(context, PathContext::Expr, q_self.as_ref(), path, shape)
             }
             PatKind::TupleStruct(ref ts) => {
-                let path_str =
-                    rewrite_path(context, PathContext::Expr, ts.qself.as_ref(), &ts.path, shape)?;
+                let path_str = rewrite_path(
+                    PathContext::Expr,
+                    PathContext::Expr,
+                    ts.qself.as_ref(),
+                    &ts.path,
+                )?;
+                )?;
                 rewrite_tuple_pat(&ts.pats, Some(path_str), self.span, context, shape)
             }
             PatKind::Lit(ref expr) => expr.rewrite(context, shape),
Mismatch at src/patterns.rs:255:
                 None,
                 None,
             ),
             ),
-            PatKind::Struct(ref s) => {
-                rewrite_struct_pat(&s.qself, &s.path, &s.fields, s.etc, self.span, context, shape)
-            }
+            PatKind::Struct(ref s) => rewrite_struct_pat(
+                &s.qself, &s.path, &s.fields, s.etc, self.span, context, shape,
+            ),
             PatKind::MacCall(ref mac) => {
                 rewrite_macro(mac, None, context, shape, MacroPosition::Pat)

Mismatch at src/attr.rs:49:
 }
 
 
 /// Returns attributes that are within `outer_span`.
-pub(crate) fn filter_inline_attrs(
-    attrs: &[ast::Attribute],
-    outer_span: Span,
-) -> ast::AttrVec {
+pub(crate) fn filter_inline_attrs(attrs: &[ast::Attribute], outer_span: Span) -> ast::AttrVec {
         .iter()
         .iter()
         .filter(|a| outer_span.lo() <= a.span.lo() && a.span.hi() <= outer_span.hi())
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
