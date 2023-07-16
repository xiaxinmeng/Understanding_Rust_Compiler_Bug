
// compile-flags: --test -Cpanic=abort -Zpanic_abort_tests
// run-flags: --test-threads=1
// run-fail
// check-run-results
// exec-env:RUST_BACKTRACE=0

// ignore-wasm no panic or subprocess support
// ignore-emscripten no panic or subprocess support
