plain
Successfully built af569dab980b
Successfully tagged rust-ci:latest
Built container sha256:af569dab980b99d4bf8229edd9e051d34a63cc36c73813549421ac950e3e8e1e
Uploading finished image to https://ci-caches.rust-lang.org/docker/93608764ca974412130ba4a304c50b0ea4a89a9b5853d488b55e530021cbeebcb5549b17247c6674f12706902ca4687aae74f60088fb03044e2d4026dd4d59e0
upload failed: - to s3://rust-lang-ci-sccache2/docker/93608764ca974412130ba4a304c50b0ea4a89a9b5853d488b55e530021cbeebcb5549b17247c6674f12706902ca4687aae74f60088fb03044e2d4026dd4d59e0 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-12]
---
doc tests for: /checkout/src/doc/rustdoc/src/the-doc-attribute.md
doc tests for: /checkout/src/doc/rustdoc/src/unstable-features.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/rustdoc/src/unstable-features.md" "--test-args" ""

stdout ----

running 7 tests
---
test /checkout/src/doc/rustdoc/src/unstable-features.md - Unstable_features::Extensions_to_the_::__Recording_what_platforms_or_features_are_required_for_code_to_be_present (line 91) ... ok

failures:

---- /checkout/src/doc/rustdoc/src/unstable-features.md - Unstable_features::Extensions_to_the_::__Recording_what_platforms_or_features_are_required_for_code_to_be_present (line 111) stdout ----
error[E0658]: `#[doc(cfg_hide)]` is experimental
  |
  |
2 | #![doc(cfg_hide(doc))]
  |
  = note: see issue #43781 <https://github.com/rust-lang/rust/issues/43781> for more information
  = note: see issue #43781 <https://github.com/rust-lang/rust/issues/43781> for more information
  = help: add `#![feature(doc_cfg_hide)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
