plain
Successfully built ea338b340e32
Successfully tagged rust-ci:latest
Built container sha256:ea338b340e32dbbebcbd8e6c2e8ecb2ed4b66bdd6523841a901105b357e784c8
Uploading finished image to https://ci-caches.rust-lang.org/docker/93608764ca974412130ba4a304c50b0ea4a89a9b5853d488b55e530021cbeebcb5549b17247c6674f12706902ca4687aae74f60088fb03044e2d4026dd4d59e0
upload failed: - to s3://rust-lang-ci-sccache2/docker/93608764ca974412130ba4a304c50b0ea4a89a9b5853d488b55e530021cbeebcb5549b17247c6674f12706902ca4687aae74f60088fb03044e2d4026dd4d59e0 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-12]
---
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 1036 tests
---
test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0788 (line 16721) - compile fail ... FAILED

failures:

---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0788 (line 16721) stdout ----
Test compiled successfully, but it's marked `compile_fail`.
failures:
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0788 (line 16721)

test result: FAILED. 1004 passed; 1 failed; 31 ignored; 0 measured; 0 filtered out; finished in 8.71s
