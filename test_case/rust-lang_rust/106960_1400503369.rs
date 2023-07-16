plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 5bef91c6e902f3bded724713bd2a64ea50abbd25 and 4c4e7b9a89e4aadabb362bbcc0dd674eeeaa9415
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

Mismatch at src/types.rs:839:
                 })
             }
             ast::TyKind::CVarArgs => Some("...".to_owned()),
-            ast::TyKind::Err => {
-                Some(context.snippet(self.span).to_owned())
-            }
+            ast::TyKind::Err => Some(context.snippet(self.span).to_owned()),
             ast::TyKind::Typeof(ref anon_const) => rewrite_call(
                 "typeof",
test test::self_tests ... FAILED
test test::system_tests ... ok
test test::idempotence_tests ... ok
test test::idempotence_tests ... ok

failures:

---- test::self_tests stdout ----
Ran 5 self tests.
thread 'test::self_tests' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`: 1 self tests failed', src/tools/rustfmt/src/test/mod.rs:400:5


failures:
    test::self_tests
