plain
Successfully built 585909f5cb1c
Successfully tagged rust-ci:latest
Built container sha256:585909f5cb1c77851906d2a863bb0945ed02af13e3efd77f3030bd25ae01d9ed
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
* 633 error codes
* highest error code: E0791
tidy error: /checkout/compiler/rustc_hir_typeck/src/fn_ctxt/checks.rs:1871: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_hir_typeck/src/fn_ctxt/checks.rs:2418: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_hir_typeck/src/fn_ctxt/checks.rs:2470: TODO is deprecated; use FIXME
some tidy checks failed
Build completed unsuccessfully in 0:00:18
