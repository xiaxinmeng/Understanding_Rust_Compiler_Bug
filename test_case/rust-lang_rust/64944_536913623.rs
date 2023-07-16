plain
2019-10-01T07:44:28.8453689Z Building test helpers
2019-10-01T07:44:28.8459934Z running: "clang" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=wasm32-unknown-unknown" "-o" "/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers/rust_test_helpers.o" "-c" "/checkout/src/test/auxiliary/rust_test_helpers.c"
2019-10-01T07:44:28.8469275Z thread 'main' panicked at '
2019-10-01T07:44:28.8469419Z 
2019-10-01T07:44:28.8469536Z Internal error occurred: Failed to find tool. Is `clang` installed?
2019-10-01T07:44:28.8469978Z ', /cargo/registry/src/github.com-1ecc6299db9ec823/cc-1.0.35/src/lib.rs:2398:5
2019-10-01T07:44:28.8471164Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-01T07:44:28.8519851Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
2019-10-01T07:44:28.8521090Z Build completed unsuccessfully in 1:11:19
2019-10-01T07:44:28.8521090Z Build completed unsuccessfully in 1:11:19
2019-10-01T07:44:28.8585963Z == clock drift check ==
2019-10-01T07:44:28.8603001Z   local time: Tue Oct  1 07:44:28 UTC 2019
2019-10-01T07:44:29.0127364Z   network time: Tue, 01 Oct 2019 07:44:29 GMT
2019-10-01T07:44:29.0127580Z == end clock drift check ==
2019-10-01T07:44:30.0385225Z ##[error]Bash exited with code '1'.
2019-10-01T07:44:30.0431477Z ##[section]Starting: Upload CPU usage statistics
2019-10-01T07:44:30.0450531Z ==============================================================================
2019-10-01T07:44:30.0450636Z Task         : Bash
2019-10-01T07:44:30.0450731Z Description  : Run a Bash script on macOS, Linux, or Windows
