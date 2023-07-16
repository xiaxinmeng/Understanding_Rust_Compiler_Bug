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
Successfully built 77f3db6f9156
Successfully tagged rust-ci:latest
Built container sha256:77f3db6f91564815c3636fd52393c401076cb2f0a4cd3993757cd24052fbe16f
Uploading finished image to https://ci-caches.rust-lang.org/docker/3fe07ec0726281b6abc4eee217b77a7a326731f1131ca615bf5ec770d1649a5f8539f46945a60baae82f6007dbca3cfd92e9dc40dae540006e8e99f677f6a4d2
upload failed: - to s3://rust-lang-ci-sccache2/docker/3fe07ec0726281b6abc4eee217b77a7a326731f1131ca615bf5ec770d1649a5f8539f46945a60baae82f6007dbca3cfd92e9dc40dae540006e8e99f677f6a4d2 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-tools]
---
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Build completed successfully in 0:02:08
+ ../x.py test src/test/rustdoc-gui
/usr/bin/env: 'python': No such file or directory
