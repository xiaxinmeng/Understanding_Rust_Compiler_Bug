plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 40336865fe7d4a01139a3336639c6971647e885c and ff5e92e06d434e0f01698d7af2ca9aebd3f730ed
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
test test::configuration_snippet::configuration_snippet_tests ... ok

Mismatch at src/expr.rs:1239:
                 format!(
                     "0x{}{}",
                     hex_lit,
-                    lit.token_lit.suffix.map_or(String::new(), |s| s.to_string())
+                    lit.token_lit
+                        .suffix
+                        .map_or(String::new(), |s| s.to_string())
                 context.config.max_width(),
                 shape,
test test::self_tests ... FAILED
test test::system_tests ... ok
---
---- test::self_tests stdout ----
Ran 5 self tests.
thread 'test::self_tests' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`: 1 self tests failed', src/tools/rustfmt/src/test/mod.rs:400:5


failures:
    test::self_tests
