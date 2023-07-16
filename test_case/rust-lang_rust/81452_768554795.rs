plain
Successfully built b5a26c80938d
Successfully tagged rust-ci:latest
Built container sha256:b5a26c80938de4f3144682676d2bdee379594261097c5a667901c21abecfcc5e
Uploading finished image to https://ci-caches.rust-lang.org/docker/80afe504501370b4d310121e20e04a989f302196b07831c4375b96e05bc067556c2046e20ab2062b28a9dc9b2ae132b37d419cc55a065dfcd25501527e829ab9
upload failed: - to s3://rust-lang-ci-sccache2/docker/80afe504501370b4d310121e20e04a989f302196b07831c4375b96e05bc067556c2046e20ab2062b28a9dc9b2ae132b37d419cc55a065dfcd25501527e829ab9 Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
    Checking merge v0.1.0
    Checking serde v1.0.118
    Checking toml v0.5.7
    Checking serde_json v1.0.59
error: unused `ExitStatus` that must be used
   --> src/bootstrap/bin/rustc.rs:182:9
    |
182 |         on_fail.status().expect("Could not run the on_fail command");
    |
    |
    = note: `-D unused-must-use` implied by `-D warnings`
    = note: this ExitStatus might represent an error that should be checked and handled
error: aborting due to previous error

error: could not compile `bootstrap`

