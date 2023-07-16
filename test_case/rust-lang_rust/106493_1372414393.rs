plain
Successfully built a2c9d06dd1e0
Successfully tagged rust-ci:latest
Built container sha256:a2c9d06dd1e0a78c39e8ea2791973138a4af9d543eaf986205399fabf31e19fe
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
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 17.99s
tidy check
tidy: Skipping binary file check, read-only filesystem
tidy error: mir-opt test files should not have dashes in them: /checkout/src/test/mir-opt/building/breakable-scope-drops.rs
* highest error code: E0791
* 392 features
some tidy checks failed
Build completed unsuccessfully in 0:00:22
