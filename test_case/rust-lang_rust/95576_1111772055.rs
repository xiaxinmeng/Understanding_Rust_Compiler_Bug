plain
Successfully built a553e5ed9816
Successfully tagged rust-ci:latest
Built container sha256:a553e5ed98161c199f0a80f01788a2ae18eccdf4f206ed7a9aa5f2ba59b4d668
Uploading finished image to https://ci-caches.rust-lang.org/docker/7a933025cb171c4b5f5b6011a3583795b8dcc545c75914dd6be8b89a1706a0990abf8b940dcd0d758184ddd671b7cd225927f49feaa72b37c79d66fed681b775
upload failed: - to s3://rust-lang-ci-sccache2/docker/7a933025cb171c4b5f5b6011a3583795b8dcc545c75914dd6be8b89a1706a0990abf8b940dcd0d758184ddd671b7cd225927f49feaa72b37c79d66fed681b775 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-12]
---
failures:

---- [ui] src/test/ui/generator/yield-in-box.rs stdout ----

error: test run failed!
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-in-box/a"
stdout: none
stderr: none


Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:
