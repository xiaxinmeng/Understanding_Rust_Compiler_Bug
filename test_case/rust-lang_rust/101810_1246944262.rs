plain
Successfully built c3ed1bc0840f
Successfully tagged rust-ci:latest
Built container sha256:c3ed1bc0840f4d98a388a0a6dd0b94bd7264ca450688545dc75215047a3738e1
Uploading finished image to https://ci-caches.rust-lang.org/docker/100443de95ac5fd5d76f947e37204434fbf3cf3da99f888ec8bb4fbbceda7ed7d93b519998f8388225735159bb6cbee56e76ad72dc699a1cf736f3af809f03d0
upload failed: - to s3://rust-lang-ci-sccache2/docker/100443de95ac5fd5d76f947e37204434fbf3cf3da99f888ec8bb4fbbceda7ed7d93b519998f8388225735159bb6cbee56e76ad72dc699a1cf736f3af809f03d0 Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
    Checking rand v0.7.3
    Checking core v0.0.0 (/checkout/library/core)
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking std v0.0.0 (/checkout/library/std)
error: to use a constant of type `std::cmp::Ordering` in a pattern, `std::cmp::Ordering` must be annotated with `#[derive(PartialEq, Eq)]`
    |
144 |         REVERSE => {}
    |         ^^^^^^^


error: unreachable pattern
   --> library/core/tests/cmp.rs:145:9
    |
144 |         REVERSE => {}
    |         ------- matches any value
145 |         _ => unreachable!(),
    |         ^ unreachable pattern
    |
    = note: `-D unreachable-patterns` implied by `-D warnings`
error: could not compile `core` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:01:22
