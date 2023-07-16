plain
doc tests for: /checkout/src/doc/rustc/src/platform-support/armv6k-nintendo-3ds.md
doc tests for: /checkout/src/doc/rustc/src/platform-support/armv7-sony-vita-newlibeabihf.md


command did not execute successfully: CFG_RELEASE_CHANNEL="nightly" RUSTC_BOOTSTRAP="1" RUSTC_STAGE="2" RUSTC_SYSROOT="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" RUSTDOC_LIBDIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" RUSTDOC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_TEST_THREADS="16" "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/rustc/src/platform-support/armv7-sony-vita-newlibeabihf.md" "--test-args" ""

stdout ----

running 1 test
running 1 test
test /checkout/src/doc/rustc/src/platform-support/armv7-sony-vita-newlibeabihf.md - armv7_sony_vita_newlibeabihf::Building (line 36) ... FAILED
failures:


---- /checkout/src/doc/rustc/src/platform-support/armv7-sony-vita-newlibeabihf.md - armv7_sony_vita_newlibeabihf::Building (line 36) stdout ----
error[E0765]: unterminated double quote string
 --> /checkout/src/doc/rustc/src/platform-support/armv7-sony-vita-newlibeabihf.md:37:89
  |
1 | cargo build -Z build-std=std,panic_abort --target=armv7-sony-vita-newlibeabihf --release"

error: aborting due to previous error

For more information about this error, try `rustc --explain E0765`.
For more information about this error, try `rustc --explain E0765`.
Couldn't compile the test.

failures:
    /checkout/src/doc/rustc/src/platform-support/armv7-sony-vita-newlibeabihf.md - armv7_sony_vita_newlibeabihf::Building (line 36)
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


stderr ----
