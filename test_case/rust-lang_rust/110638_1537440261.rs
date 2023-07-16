plain
doc tests for: /checkout/src/doc/rustc/src/platform-support/armv6k-nintendo-3ds.md
doc tests for: /checkout/src/doc/rustc/src/platform-support/armv7-sony-vita-newlibeabihf.md


command did not execute successfully: CFG_RELEASE_CHANNEL="nightly" RUSTC_BOOTSTRAP="1" RUSTC_STAGE="2" RUSTC_SYSROOT="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" RUSTDOC_LIBDIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" RUSTDOC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_TEST_THREADS="16" "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/rustc/src/platform-support/armv7-sony-vita-newlibeabihf.md" "--test-args" ""

stdout ----

running 1 test
running 1 test
test /checkout/src/doc/rustc/src/platform-support/armv7-sony-vita-newlibeabihf.md - armv7_sony_vita_newlibeabihf::Building_and_Running_Rust_Programs (line 65) ... FAILED
failures:

Build completed unsuccessfully in 0:29:26
Build completed unsuccessfully in 0:29:26
---- /checkout/src/doc/rustc/src/platform-support/armv7-sony-vita-newlibeabihf.md - armv7_sony_vita_newlibeabihf::Building_and_Running_Rust_Programs (line 65) stdout ----
error[E0580]: `main` function has wrong type
 --> /checkout/src/doc/rustc/src/platform-support/armv7-sony-vita-newlibeabihf.md:72:1
8 | pub extern "C" fn main() {
8 | pub extern "C" fn main() {
  | ^^^^^^^^^^^^^^^^^^^^^^^^ expected "Rust" fn, found "C" fn
  = note: expected fn pointer `fn()`
             found fn pointer `extern "C" fn()`

error: aborting due to previous error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0580`.
Couldn't compile the test.

failures:
    /checkout/src/doc/rustc/src/platform-support/armv7-sony-vita-newlibeabihf.md - armv7_sony_vita_newlibeabihf::Building_and_Running_Rust_Programs (line 65)
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s


stderr ----
