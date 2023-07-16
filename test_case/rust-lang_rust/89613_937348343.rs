plain
Successfully built 7771d0d28550
Successfully tagged rust-ci:latest
Built container sha256:7771d0d285502b72556fe9a33780fbb4f9cb3c64e565556918bb996651b66e85
Uploading finished image to https://ci-caches.rust-lang.org/docker/f660c3ad6bae343070b47dc83125c3e2707aea949eaba6c703e6e712f1d5f695815823062f0ccc90ed4dc634b4c2097c53175bcc314ac8405b0a16c0fd2aa018
upload failed: - to s3://rust-lang-ci-sccache2/docker/f660c3ad6bae343070b47dc83125c3e2707aea949eaba6c703e6e712f1d5f695815823062f0ccc90ed4dc634b4c2097c53175bcc314ac8405b0a16c0fd2aa018 Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
    Checking cargo_metadata v0.12.0
    Checking rustfix v0.5.1
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
    Checking clippy_lints v0.1.57 (/checkout/src/tools/clippy/clippy_lints)
error[E0609]: no field `dylib` on type `Lrc<CrateSource>`
    |
    |
367 |                 if let Some(ref src) = source.dylib {

error[E0609]: no field `0` on type `&_`
   --> src/tools/clippy/clippy_lints/src/utils/inspector.rs:368:69
    |
    |
368 |                     println!("extern crate dylib source: {:?}", src.0);

error[E0609]: no field `rlib` on type `Lrc<CrateSource>`
   --> src/tools/clippy/clippy_lints/src/utils/inspector.rs:370:47
    |
    |
370 |                 if let Some(ref src) = source.rlib {

error[E0609]: no field `0` on type `&_`
   --> src/tools/clippy/clippy_lints/src/utils/inspector.rs:371:68
    |
    |
371 |                     println!("extern crate rlib source: {:?}", src.0);

For more information about this error, try `rustc --explain E0609`.
error: could not compile `clippy_lints` due to 4 previous errors
Build completed unsuccessfully in 0:03:57
