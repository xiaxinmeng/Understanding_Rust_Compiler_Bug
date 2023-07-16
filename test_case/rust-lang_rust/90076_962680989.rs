plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 68568dcb8ff49a3d70f4cc2d9215b5753d088738 and d7fc0eb4dc77c8bbb9e868fcfbf3c964f77a04d2
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
Mismatch at src/items.rs:1945:
             span,
         )
     } else {
-        rewrite_type(context, indent, ident, vis, generics, where_clauses, where_predicates_split, None, ty_opt, span)
+            context,
+            indent,
+            ident,
+            vis,
---
---- test::self_tests stdout ----
Ran 5 self tests.
thread 'test::self_tests' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`: 1 self tests failed', src/tools/rustfmt/src/test/mod.rs:357:5


failures:
    test::self_tests
