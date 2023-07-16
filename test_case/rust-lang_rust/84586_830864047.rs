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
Successfully built eb0faf2e9bff
Successfully tagged rust-ci:latest
Built container sha256:eb0faf2e9bffd9dbc10a5ffd3f58b33a9cf899bf5c0caf812369f2dc5d450710
Uploading finished image to https://ci-caches.rust-lang.org/docker/a1780e92a2543e89f8c7737d41de5dccc35b4e4e06bae59a60ca8806a8b1f247f32152041604ffcdc66ca0c3b79bc1caa2796c55228263cf03beba351f5733b3
upload failed: - to s3://rust-lang-ci-sccache2/docker/a1780e92a2543e89f8c7737d41de5dccc35b4e4e06bae59a60ca8806a8b1f247f32152041604ffcdc66ca0c3b79bc1caa2796c55228263cf03beba351f5733b3 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-tools]
---
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Build completed successfully in 0:01:47
+ ../x.py test src/test/rustdoc-gui
/usr/bin/env: 'python': No such file or directory
