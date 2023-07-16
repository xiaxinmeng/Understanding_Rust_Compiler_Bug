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
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between ee5ed4a88d6a869cdb152829ed697d6459650db3 and 4ac31063ff02c05793ae8dfea5be332d5806fd95
Executing the job since rustdoc was updated
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
Successfully built cd9e59a8c518
Successfully tagged rust-ci:latest
Built container sha256:cd9e59a8c5184e8bd4f912ac5433fd39575ca643464407f9cacfbe978ce6da97
Uploading finished image to https://ci-caches.rust-lang.org/docker/aebd8caf54dd5d7b5b1092ca66f2cea9bc9dd285021d1c989b7a21ef3404ac6ce78ee86f7c7940a5d26725945c879b9e4a9288e07a69eb9864c1ee345fb06d65
upload failed: - to s3://rust-lang-ci-sccache2/docker/aebd8caf54dd5d7b5b1092ca66f2cea9bc9dd285021d1c989b7a21ef3404ac6ce78ee86f7c7940a5d26725945c879b9e4a9288e07a69eb9864c1ee345fb06d65 Unable to locate credentials
 * branch                    master     -> FETCH_HEAD
warning: 969c145f96045a156a5776a8f4407e87df5a07b6:.gitmodules, multiple configurations found for 'submodule.src/rust-installer.path'. Skipping second one!
warning: 969c145f96045a156a5776a8f4407e87df5a07b6:.gitmodules, multiple configurations found for 'submodule.src/rust-installer.url'. Skipping second one!
warning: 969c145f96045a156a5776a8f4407e87df5a07b6:.gitmodules, multiple configurations found for 'submodule.src/doc/nomicon.path'. Skipping second one!
---
From https://github.com/rust-lang/miri
 * branch              062cf07d98822beb4c5b1132f47b4397275f403a -> FETCH_HEAD
 * branch              06758c48bd7a77bb5aa43fc50cf344540ba5afef -> FETCH_HEAD
 * branch              dd7f545a69e4b720407e458bf4ade0b207bbf9ee -> FETCH_HEAD
fatal: remote error: upload-pack: not our ref 9ad6e5b8f68ee4bcd85886e50b8b0a70cbb91a52
Errors during submodule fetch:
##[error]Process completed with exit code 1.
Post job cleanup.
