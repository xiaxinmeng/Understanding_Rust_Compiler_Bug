plain
Successfully built 13be3f5b8e2c
Successfully tagged rust-ci:latest
Built container sha256:13be3f5b8e2ced24ea093af4bac86538ddad994d189fed6a1b2366664b5b3def
Uploading finished image to https://ci-caches.rust-lang.org/docker/b9f294a0befed8b17c2bc42b05ee41fee9e3a4f406488e62711cb977b54d0c8af064f18c6f8b6489a1ca014e690056319cefd5fd29a4e2b96167954fda9f23f4
upload failed: - to s3://rust-lang-ci-sccache2/docker/b9f294a0befed8b17c2bc42b05ee41fee9e3a4f406488e62711cb977b54d0c8af064f18c6f8b6489a1ca014e690056319cefd5fd29a4e2b96167954fda9f23f4 Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
* highest error code: E0787
Found 504 error codes
Found 0 error(s) in error codes
Done!
tidy error: /checkout/src/ci/pgo.sh:86: line longer than 100 chars
some tidy checks failed
Build completed unsuccessfully in 0:00:12
