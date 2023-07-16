plain
Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building test helpers
running: "cc" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers/rust_test_helpers.o" "-c" "/checkout/src/test/auxiliary/rust_test_helpers.c"
cargo:warning=/checkout/src/test/auxiliary/rust_test_helpers.c:13:29: error: unknown type name 'dbg_callback'
cargo:warning= rust_dbg_identitiy_callback(dbg_callback cb, void *data) {
cargo:warning=                             ^~~~~~~~~~~~
exit code: 1


error occurred: Command "cc" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers/rust_test_helpers.o" "-c" "/checkout/src/test/auxiliary/rust_test_helpers.c" with args "cc" did not execute successfully (status code exit code: 1).

failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:24
