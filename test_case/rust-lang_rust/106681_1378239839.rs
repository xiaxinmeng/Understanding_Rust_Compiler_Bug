plain
Successfully built 050747f69934
Successfully tagged rust-ci:latest
Built container sha256:050747f6993403997e50955f3fdfa895bb534fef75fbc063f41bfe21db96c0de
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
tidy: Skipping binary file check, read-only filesystem
Found 505 error codes
Highest error code: `E0791`
* 392 features
thread '<unnamed>' panicked at 'could not read lib feature file /checkout/src/doc/unstable-book/src/library-features/allocator_api.md: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/tools/tidy/src/unstable_book.rs:195:54
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'a scoped thread panicked', src/tools/tidy/src/main.rs:54:5
