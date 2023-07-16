plain
Successfully built 90d710a92abd
Successfully tagged rust-ci:latest
Built container sha256:90d710a92abd2ecf51bec5af0202e13dd51a03f8262624ebf87b5911142e0bcc
Uploading finished image to https://ci-caches.rust-lang.org/docker/f660c3ad6bae343070b47dc83125c3e2707aea949eaba6c703e6e712f1d5f695815823062f0ccc90ed4dc634b4c2097c53175bcc314ac8405b0a16c0fd2aa018
upload failed: - to s3://rust-lang-ci-sccache2/docker/f660c3ad6bae343070b47dc83125c3e2707aea949eaba6c703e6e712f1d5f695815823062f0ccc90ed4dc634b4c2097c53175bcc314ac8405b0a16c0fd2aa018 Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
error[E0599]: no variant or associated item named `Unwind` found for enum `OomStrategy` in the current scope
    |
    |
744 |     tracked!(oom, OomStrategy::Unwind);
    |                                ^^^^^^ variant or associated item not found in `OomStrategy`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_interface` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
