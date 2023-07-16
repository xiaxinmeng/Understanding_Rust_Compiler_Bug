plain
Successfully built c601cfcfb733
Successfully tagged rust-ci:latest
Built container sha256:c601cfcfb7332432d663cb66b7b3d107735749d79cbe6dbcfa9b74a07a89884d
Uploading finished image to https://ci-caches.rust-lang.org/docker/3ba9d538a45014cf9c069a7f7b39b17975213bb4ad3cb92953bcf42e2feac7246274891423ce6ec10459580375996bc5323b9b2e1eaa3f9fe83d7d3a2f2335cf
upload failed: - to s3://rust-lang-ci-sccache2/docker/3ba9d538a45014cf9c069a7f7b39b17975213bb4ad3cb92953bcf42e2feac7246274891423ce6ec10459580375996bc5323b9b2e1eaa3f9fe83d7d3a2f2335cf Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-13]
---
   Compiling cfg-if v0.1.10
   Compiling cfg-if v1.0.0
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.21
error: ~const can only be applied to `#[const_trait]` traits
    |
    |
539 |         T: ~const ToOwned,


error: ~const can only be applied to `#[const_trait]` traits
    |
    |
575 |         T: ~const ToOwned,


error: ~const can only be applied to `#[const_trait]` traits
    |
    |
523 |         T: ~const ToOwned,


error: ~const can only be applied to `#[const_trait]` traits
    |
    |
558 |         T: ~const ToOwned,

error: could not compile `alloc` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:04:11
