plain
Successfully built 85a2956380db
Successfully tagged rust-ci:latest
Built container sha256:85a2956380dbc45abc087cfd7dd4a9b51c66d2d44bdfbfe1627afe326676f3cf
Uploading finished image to https://ci-caches.rust-lang.org/docker/d33e4deadd55ac310bf6d14046967e2ae76df1d0cbcfb3cbdfd57a515110e5c43bea0fc68be40129a2ca62a8c3c0cddee210b82b000006fe1b3d251a4634baf3
upload failed: - to s3://rust-lang-ci-sccache2/docker/d33e4deadd55ac310bf6d14046967e2ae76df1d0cbcfb3cbdfd57a515110e5c43bea0fc68be40129a2ca62a8c3c0cddee210b82b000006fe1b3d251a4634baf3 Unable to locate credentials
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
Found 503 error codes
Highest error code: `E0791`
tidy error: following path contains more than 939 entries, you should move the test to some relevant subdirectory (current: 941): /checkout/tests/ui
some tidy checks failed
Build completed unsuccessfully in 0:00:19
