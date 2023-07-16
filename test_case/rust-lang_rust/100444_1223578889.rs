plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 7e3e8a816f91fda5c6e05c659d7038747aff0b55 and cb4d5e18b62cdba46fc705de93e59789c7604325
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
test test::verify_config_test_names ... ok
test test::verify_check_works ... ok
test test::configuration_snippet::configuration_snippet_tests ... ok

Mismatch at src/expr.rs:203:
                 Some("yield".to_string())
         }
         }
-        ast::ExprKind::Closure(ref cl) => {
-            closures::rewrite_closure(
-                &cl.binder,
-                cl.capture_clause,
-                &cl.asyncness,
-                cl.movability,
-                &cl.fn_decl,
-                &cl.body,
-                expr.span,
-                shape,
-            )
-        }
-        }
+        ast::ExprKind::Closure(ref cl) => closures::rewrite_closure(
+            &cl.binder,
+            cl.capture_clause,
+            &cl.asyncness,
+            cl.movability,
+            &cl.fn_decl,
+            &cl.body,
+            expr.span,
+            shape,
+        ),
         ast::ExprKind::Try(..)
         ast::ExprKind::Try(..)
         | ast::ExprKind::Field(..)
         | ast::ExprKind::MethodCall(..)
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
