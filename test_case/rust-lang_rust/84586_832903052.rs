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
Step 10/11 : ENV RUST_CONFIGURE_ARGS   --build=x86_64-unknown-linux-gnu   --save-toolstates=/tmp/toolstate/toolstates.json
 ---> Running in 6de4cb9f6a31
Removing intermediate container 6de4cb9f6a31
 ---> 304d786f2a14
Step 11/11 : ENV SCRIPT /tmp/checktools.sh ../x.py &&   npm install browser-ui-test -g --unsafe-perm=true &&   python3 ../x.py test src/test/rustdoc-gui --stage 2
Removing intermediate container a8c914afa1ff
 ---> c3c9613ee128
Successfully built c3c9613ee128
Successfully tagged rust-ci:latest
Successfully tagged rust-ci:latest
Built container sha256:c3c9613ee12831d1def321e75343ec81c515a5fdf4e7f2b7e89aa733853296c9
Uploading finished image to https://ci-caches.rust-lang.org/docker/60a276d2b872f175f1501d2058a771339743a78912274b5d37dddbe884ca2792e329de3d23c1890f1b702f13713516849903bf85cfe627672a533d76a808029d
upload failed: - to s3://rust-lang-ci-sccache2/docker/60a276d2b872f175f1501d2058a771339743a78912274b5d37dddbe884ca2792e329de3d23c1890f1b702f13713516849903bf85cfe627672a533d76a808029d Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-tools]
---
+ python3 ../x.py test src/test/rustdoc-gui --stage 2
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
error: rustdoc-gui test suite cannot be run because npm `browser-ui-test` dependency is missing
If you want to install the `browser-ui-test` dependency, run `npm install browser-ui-test`
thread 'main' panicked at 'Cannot run rustdoc-gui tests', src/bootstrap/test.rs:829:13
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/rustdoc-gui --stage 2
Build completed unsuccessfully in 0:00:00
