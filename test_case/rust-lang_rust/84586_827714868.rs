plain
Successfully built 32611eab603e
Successfully tagged rust-ci:latest
Built container sha256:32611eab603e63ce9e6e0b930183f54f214d2c897a64a8cd21e862a11cf3a93e
Uploading finished image to https://ci-caches.rust-lang.org/docker/65f96c85b21baa4f053862a0c855e5cd16d8d32069c40bbaa6c5f098733cd3db3acb0af07f0f8dee730a01bce944a90252ff29fadf973cc190909b1bfa037225
upload failed: - to s3://rust-lang-ci-sccache2/docker/65f96c85b21baa4f053862a0c855e5cd16d8d32069c40bbaa6c5f098733cd3db3acb0af07f0f8dee730a01bce944a90252ff29fadf973cc190909b1bfa037225 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-10]
---
Set({"src/tools/lint-docs"}) not skipped for "bootstrap::test::LintDocs" -- not in ["src/tools/tidy"]
Set({"src/test/rustdoc-js-std"}) not skipped for "bootstrap::test::RustdocJSStd" -- not in ["src/tools/tidy"]
Set({"src/test/rustdoc-js"}) not skipped for "bootstrap::test::RustdocJSNotStd" -- not in ["src/tools/tidy"]
Set({"src/test/rustdoc-gui"}) not skipped for "bootstrap::test::RustdocGUI" -- not in ["src/tools/tidy"]
thread 'main' panicked at 'Cannot run rustdoc-gui tests', src/bootstrap/test.rs:809:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
If you want to install the `browser-ui-test` dependency, run `npm install browser-ui-test`
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:00:01
