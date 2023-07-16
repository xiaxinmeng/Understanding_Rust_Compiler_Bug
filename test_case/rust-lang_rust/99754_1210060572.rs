plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 34a6cae28e7013ff0e640026a8e46f315426829d and 6eb72bee84c9150566020648f3920dad5bf0011c
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
test test::verify_check_works_with_stdin ... ok
test test::verify_config_test_names ... ok
test test::configuration_snippet::configuration_snippet_tests ... ok

Mismatch at src/types.rs:800:
                 rewrite_tuple(context, items.iter(), self.span, shape, items.len() == 1)
             }
             ast::TyKind::AnonymousStruct(_, _) => Some(context.snippet(self.span).to_owned()),
+
+
             ast::TyKind::AnonymousUnion(_, _) => Some(context.snippet(self.span).to_owned()),
+
+
             ast::TyKind::Path(ref q_self, ref path) => {
                 rewrite_path(context, PathContext::Type, q_self.as_ref(), path, shape)
test test::self_tests ... FAILED
test test::system_tests ... ok
test test::idempotence_tests ... ok


failures:

---- test::self_tests stdout ----
Ran 5 self tests.
thread 'test::self_tests' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`: 1 self tests failed', src/tools/rustfmt/src/test/mod.rs:400:5


failures:
    test::self_tests
