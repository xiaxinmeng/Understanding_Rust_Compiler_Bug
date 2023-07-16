plain
Successfully built 503872618c25
Successfully tagged rust-ci:latest
Built container sha256:503872618c2523db0277f5d6bb5797b3c6ab31e6a7d365b4a333d425447ccc76
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
tidy error: /checkout/compiler/rustc_resolve/src/build_reduced_graph.rs:859: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_feature/src/active.rs:419: no tracking issue for feature impl_restriction
tidy error: /checkout/compiler/rustc_feature/src/active.rs:455: no tracking issue for feature mut_restriction
tidy error: /checkout/compiler/rustc_error_messages/src/lib.rs:71: line not in alphabetical order
some tidy checks failed
