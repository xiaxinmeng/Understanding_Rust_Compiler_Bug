plain
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "CI_ONLY_WHEN_SUBMODULES_CHANGED": 1
##[endgroup]
adding extra environment variable CI_ONLY_WHEN_SUBMODULES_CHANGED
linux builder detected, using docker to run the build
##[group]Run src/ci/scripts/should-skip-this.sh
---
 ---> 42ee713cc18b
Successfully built 42ee713cc18b
Successfully tagged rust-ci:latest
Built container sha256:42ee713cc18bc3bcee17ff3fd0654b1a23711449b335bdcda2913d4b1c8557a5
Uploading finished image to https://ci-caches.rust-lang.org/docker/3fe194013521c365e334126a9e48153cebe500914e1f453338539e45a3f82b90714bb596a83b49e4d7d47df65da08149edebbb91d1517ea0220ea8e6a7bca88b
upload failed: - to s3://rust-lang-ci-sccache2/docker/3fe194013521c365e334126a9e48153cebe500914e1f453338539e45a3f82b90714bb596a83b49e4d7d47df65da08149edebbb91d1517ea0220ea8e6a7bca88b Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-tools]
---
+ python3 ../x.py test src/test/rustdoc-gui --stage 2
    Finished dev [unoptimized + debuginfo] target(s) in 0.20s
error: rustdoc-gui test suite cannot be run because npm `browser-ui-test` dependency is missing
If you want to install the `browser-ui-test` dependency, run `npm install browser-ui-test`
thread 'main' panicked at 'Cannot run rustdoc-gui tests', src/bootstrap/test.rs:829:13
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/rustdoc-gui --stage 2
Build completed unsuccessfully in 0:00:00
