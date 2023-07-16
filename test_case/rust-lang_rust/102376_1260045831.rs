plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 57ee5cf5a93923dae9c98bffb11545fc3a31368d and 7097e2a39a4bf6add2fffbd0741ed05d75d1f61a
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
test test::verify_check_l_works_with_stdin ... ok
test unit_tests::test_format_code_block ... ok
test test::configuration_snippet::configuration_snippet_tests ... ok

Mismatch at src/items.rs:3244:
             ast::ForeignItemKind::MacCall(ref mac) => {
                 rewrite_macro(mac, None, context, shape, MacroPosition::Item)
             }
-            ast::ForeignItemKind::Impl(ref impl_) => {
-                rewrite_impl(context, &self.vis, self.span, &self.attrs, impl_, shape.indent)
-            }
+            ast::ForeignItemKind::Impl(ref impl_) => rewrite_impl(
+                &self.vis,
+                self.span,
+                self.span,
+                &self.attrs,
+                impl_,
+                shape.indent,
         }?;
 
 
         let missing_span = if self.attrs.is_empty() {
test test::system_tests ... ok
test test::idempotence_tests ... ok

failures:
failures:

---- test::self_tests stdout ----
Ran 5 self tests.
thread 'test::self_tests' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`: 1 self tests failed', src/tools/rustfmt/src/test/mod.rs:400:5
error: test failed, to rerun pass `--lib`


failures:
