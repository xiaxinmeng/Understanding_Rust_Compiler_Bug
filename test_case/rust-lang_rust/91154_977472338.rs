plain
    |
note: function defined here
   --> /checkout/src/tools/clippy/clippy_lints/src/lib.rs:414:8
    |
414 | pub fn register_pre_expansion_lints(store: &mut rustc_lint::LintStore, sess: &Session, conf: &Conf) {

For more information about this error, try `rustc --explain E0061`.
[RUSTC-TIMING] rls_rustc test:false 0.420
error: could not compile `rls-rustc` due to previous error
---
Verifying status of rls...
Verifying status of miri...
Verifying status of embedded-book...
Cloning into 'rust-toolstate'...
error: Tool `rls` has regressed from test-pass to build-fail during beta week.
{"rust-by-example":"test-pass","rustbook":"test-fail","embedded-book":"test-pass","book":"test-pass","edition-guide":"test-pass","reference":"test-pass","nomicon":"test-pass","miri":"test-pass","cargo-miri":"test-fail","rls":"build-fail"}Build completed unsuccessfully in 0:00:00
