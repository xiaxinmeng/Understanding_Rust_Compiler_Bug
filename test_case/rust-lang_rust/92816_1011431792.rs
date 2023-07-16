plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 72e74d7b9cf1a7901650227e74650f1fcc797600 and 8383b1ed9a04dedea99f000a2026245609aa80cb
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
test test::verify_config_test_names ... ok
test test::verify_check_works ... ok
test test::configuration_snippet::configuration_snippet_tests ... ok

Mismatch at src/expr.rs:334:
         // satisfy our width restrictions.
         // Style Guide RFC for InlineAsm variant pending
         // https://github.com/rust-dev-tools/fmt-rfcs/issues/152
-        ast::ExprKind::InlineAsm(..) => {
-            Some(context.snippet(expr.span).to_owned())
-        }
+        ast::ExprKind::InlineAsm(..) => Some(context.snippet(expr.span).to_owned()),
         ast::ExprKind::TryBlock(ref block) => {
             if let rw @ Some(_) =
                 rewrite_single_line_block(context, "try ", block, Some(&expr.attrs), None, shape)
test test::system_tests ... ok
test test::idempotence_tests ... ok

failures:
failures:

---- test::self_tests stdout ----
Ran 5 self tests.
thread 'test::self_tests' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`: 1 self tests failed', src/tools/rustfmt/src/test/mod.rs:354:5


failures:
    test::self_tests
