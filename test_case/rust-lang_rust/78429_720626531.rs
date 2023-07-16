
     Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
 Documenting rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `LangString { original: "ignore(cross-crate-imports)", should_panic: false, no_run: false, ignore: None, rust: false, test_harness: false, compile_fail: false, error_codes: [], allow_fail: false, edition: None }`,
 right: `LangString { original: "ignore(cross-crate-imports)", should_panic: false, no_run: false, ignore: All, rust: true, test_harness: false, compile_fail: false, error_codes: [], allow_fail: false, edition: None }`: New token split differs from old token split', src/librustdoc/html/markdown.rs:829:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
