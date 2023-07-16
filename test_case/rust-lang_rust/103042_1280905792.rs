plain
test test::configuration_snippet::configuration_snippet_tests ... ok

Mismatch at src/parse/session.rs:125:
     } else {
         let fallback_bundle = rustc_errors::fallback_fluent_bundle(
             rustc_driver::DEFAULT_LOCALE_RESOURCES.to_vec(),
+            false,
         );
         );
         Box::new(EmitterWriter::stderr(
             color_cfg,
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
