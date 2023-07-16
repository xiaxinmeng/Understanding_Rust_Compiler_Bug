plain
Successfully built d70d8377c299
Successfully tagged rust-ci:latest
Built container sha256:d70d8377c29951268b81915ef564998c13058de095e34b88e7117492ea3901b5
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
   Compiling cargo_metadata v0.14.0
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 18.33s
tidy check
tidy error: Tests have been moved, please move them from /checkout/src/test to /checkout/tests
Found 503 error codes
Highest error code: `E0791`
* 394 features
some tidy checks failed
