plain
4 |     } else {
  |     ^ mismatched closing delimiter

..............................................................................
Mismatch at src/bin/main.rs:205:
     let matches = match rustc_driver::args::raw_args() {
         Ok(args) => opts.parse(args.into_iter().skip(1))?,
         // Error message has already been printed, so just return an error code
-        Err(_) => return Ok(rustc_driver::EXIT_FAILURE)
+        Err(_) => return Ok(rustc_driver::EXIT_FAILURE),
     };
     let options = GetOptsOptions::from_matches(&matches)?;
{ "type": "test", "name": "test::configuration_snippet::configuration_snippet_tests", "event": "ok" }
F..

failures:
failures:

---- test::self_tests stdout ----
Ran 5 self tests.
thread 'test::self_tests' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`: 1 self tests failed', src/tools/rustfmt/src/test/mod.rs:400:5


failures:
    test::self_tests
