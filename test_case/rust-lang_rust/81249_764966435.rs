plain
Successfully built 804340b28d80
Successfully tagged rust-ci:latest
Built container sha256:804340b28d80303e43d577e7667ab6196a15952d0b85659990a26299ff52fb3a
Uploading finished image to https://ci-caches.rust-lang.org/docker/aa85b52f727783ce661c5275c3edac7e8b4fbba025fc0a8e6392d6566595f3d157bb7da4c70e2eda4e5e6e7455d8337e7dfb6de6b288fcd6e68cf37f838a32b6
upload failed: - to s3://rust-lang-ci-sccache2/docker/aa85b52f727783ce661c5275c3edac7e8b4fbba025fc0a8e6392d6566595f3d157bb7da4c70e2eda4e5e6e7455d8337e7dfb6de6b288fcd6e68cf37f838a32b6 Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
   Compiling regex v1.4.3
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 10.00s
tidy check
tidy error: following path contains more than 2669 entries, you should move the test to some relevant subdirectory (current: 2671): /checkout/src/test/ui/issues
Found 435 error codes
Found 0 error codes with no tests
Done!
some tidy checks failed
