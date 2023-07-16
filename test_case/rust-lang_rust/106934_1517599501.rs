plain
...............................................................................
Mismatch at src/expr.rs:399:
             }
         }
         ast::ExprKind::Underscore => Some("_".to_owned()),
-        ast::ExprKind::FormatArgs(..) | ast::ExprKind::IncludedBytes(..) | ast::ExprKind::OffsetOf(..) => {
+        ast::ExprKind::FormatArgs(..)
+        | ast::ExprKind::IncludedBytes(..)
+        | ast::ExprKind::OffsetOf(..) => {
             // These do not occur in the AST because macros aren't expanded.
             unreachable!()
         }
{ "type": "test", "name": "test::self_tests", "event": "failed", "stdout": "Ran 5 self tests.\nthread 'test::self_tests' panicked at 'assertion failed: `(left == right)`\n  left: `1`,\n right: `0`: 1 self tests failed', src/tools/rustfmt/src/test/mod.rs:400:5\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n" }

test result: FAILED. 169 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 658.75ms

error: test failed, to rerun pass `--lib`
