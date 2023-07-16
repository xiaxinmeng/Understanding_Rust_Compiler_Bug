plain
doc tests for: /checkout/src/doc/rustdoc/src/scraped-examples.md
doc tests for: /checkout/src/doc/rustdoc/src/unstable-features.md


command did not execute successfully: CFG_RELEASE_CHANNEL="nightly" RUSTC_BOOTSTRAP="1" RUSTC_STAGE="2" RUSTC_SYSROOT="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" RUSTDOC_LIBDIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" RUSTDOC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_TEST_THREADS="16" "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/rustdoc/src/unstable-features.md" "--test-args" ""

stdout ----

running 7 tests
---
  |
2 | #![feature(rustdoc_internals)]
  |            ^^^^^^^^^^^^^^^^^
  |
  = note: using it is strongly discouraged
  = note: using it is strongly discouraged
  = note: `#[deny(internal_features)]` on by default
error: aborting due to previous error

Couldn't compile the test.

