
2020-07-10T17:33:01.0331851Z Set({"src/libpanic_abort"}) not skipped for "bootstrap::test::Crate" -- not in ["src/libcore", "src/liballoc", "src/libproc_macro", "src/libstd", "src/libterm", "src/libtest"]
2020-07-10T17:33:01.0346894Z thread 'main' panicked at 'nodejs not configured', src/bootstrap/test.rs:1762:17
2020-07-10T17:33:01.0347403Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-07-10T17:33:01.0362643Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-emscripten --exclude src/libcore --exclude src/liballoc --exclude src/libproc_macro --exclude src/libstd --exclude src/libterm --exclude src/libtest
