plain
Successfully built 2de1d61fe63b
Successfully tagged rust-ci:latest
Built container sha256:2de1d61fe63b91fcd6b59e203f9a84a5003c77cbae05efff2ae0be32bdd538af
Uploading finished image to https://ci-caches.rust-lang.org/docker/80afe504501370b4d310121e20e04a989f302196b07831c4375b96e05bc067556c2046e20ab2062b28a9dc9b2ae132b37d419cc55a065dfcd25501527e829ab9
upload failed: - to s3://rust-lang-ci-sccache2/docker/80afe504501370b4d310121e20e04a989f302196b07831c4375b96e05bc067556c2046e20ab2062b28a9dc9b2ae132b37d419cc55a065dfcd25501527e829ab9 Unable to locate credentials
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
    Finished release [optimized] target(s) in 10.59s
tidy check
tidy error: following path contains more than 2669 entries, you should move the test to some relevant subdirectory (current: 2671): /checkout/src/test/ui/issues
Found 435 error codes
Found 0 error codes with no tests
Done!
some tidy checks failed
