plain
Successfully built 8dc434218837
Successfully tagged rust-ci:latest
Built container sha256:8dc43421883719b107b478ea862c22a056d6b90c9025a470e6a2af880801c3b1
Uploading finished image to https://ci-caches.rust-lang.org/docker/58bd69d3781a382eb5941028d9596b84692bd50bf9ab53a2d16b350022deca32f83037b6d64da3bfdf69e2b275e0db2ddd3212390c690549c0b3382192828b45
upload failed: - to s3://rust-lang-ci-sccache2/docker/58bd69d3781a382eb5941028d9596b84692bd50bf9ab53a2d16b350022deca32f83037b6d64da3bfdf69e2b275e0db2ddd3212390c690549c0b3382192828b45 Unable to locate credentials
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
    Finished release [optimized] target(s) in 16.28s
tidy check
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
tidy error: following path contains more than 939 entries, you should move the test to some relevant subdirectory (current: 940): /checkout/src/test/ui
* highest error code: E0791
Found 507 error codes
Found 0 error(s) in error codes
Done!
