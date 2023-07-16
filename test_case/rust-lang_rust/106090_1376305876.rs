plain
Successfully built 4fd6cfdd6c3c
Successfully tagged rust-ci:latest
Built container sha256:4fd6cfdd6c3c5862d9a3fb76ada230dcb9cca3950005c53f9ff1fcdd3c045361
Uploading finished image to https://ci-caches.rust-lang.org/docker/b8cdc76b9b2adf6aaac1172d56fab8a31470587d5348a0014d9e27d5bcbbe8577c6dad29501e9359634fe46a53fc6079832ca8161c7036f56b2737f782f779b6
upload failed: - to s3://rust-lang-ci-sccache2/docker/b8cdc76b9b2adf6aaac1172d56fab8a31470587d5348a0014d9e27d5bcbbe8577c6dad29501e9359634fe46a53fc6079832ca8161c7036f56b2737f782f779b6 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
tidy check
tidy: Skipping binary file check, read-only filesystem
Found 505 error codes
Highest error code: `E0791`
tidy error: /checkout/compiler/rustc_mir_build/src/thir/pattern/deconstruct_pat.rs:152: trailing whitespace
tidy error: /checkout/compiler/rustc_mir_build/src/build/expr/stmt.rs:116: trailing whitespace
some tidy checks failed
Build completed unsuccessfully in 0:00:22
