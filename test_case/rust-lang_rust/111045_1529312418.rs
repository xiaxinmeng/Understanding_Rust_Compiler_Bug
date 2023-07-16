plain
2 +     if bar && !baz {
  |

...............................................................................
Mismatch at src/imports.rs:481:
                     result.path.push(UseSegment { kind, version });
                 let kind = UseSegmentKind::List(
-                    nested.items
+                    nested
+                        .items
+                        .items
                         .iter()
                         .zip(items)
                         .map(|(t, list_item)| {
{ "type": "test", "name": "test::self_tests", "event": "failed", "stdout": "Ran 5 self tests.\nthread 'test::self_tests' panicked at 'assertion failed: `(left == right)`\n  left: `1`,\n right: `0`: 1 self tests failed', src/tools/rustfmt/src/test/mod.rs:400:5\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n" }

test result: FAILED. 169 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 666.37ms

error: test failed, to rerun pass `--lib`
