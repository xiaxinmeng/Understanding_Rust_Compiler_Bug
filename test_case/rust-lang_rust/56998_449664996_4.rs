
     Running build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/deps/alloc-4f5d60f9094e66f5

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 100 filtered out

     Running build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/deps/collectionstests-afe1c7c2f5f7578e

running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 560 filtered out

   Doc-tests alloc
error: failed to find a `codegen-backends` folder in the sysroot candidates:
* /d/rust/build/x86_64-apple-darwin/stage0-tools/x86_64-apple-darwin
* /d/rust/build/x86_64-apple-darwin/stage0-sysroot/lib/rustlib/x86_64-apple-darwin
* /d/rust/build/x86_64-apple-darwin/stage0-sysroot

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/d/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "test" "--target" "x86_64-apple-darwin" "-j" "8" "--release" "--features" "panic-unwind backtrace" "--manifest-path" "/d/rust/src/libstd/Cargo.toml" "-p" "alloc" "--" "to_nonnull" "--quiet"
expected success, got: exit code: 1


failed to run: /d/rust/build/bootstrap/debug/bootstrap test --stage 0 src/liballoc --test-args to_nonnull
Build completed unsuccessfully in 0:08:46
