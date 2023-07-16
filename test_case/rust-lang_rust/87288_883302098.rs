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
warning: e9a64996bedbc224d1edb401162d2c3cc15eb7c1:.gitmodules, multiple configurations found for 'submodule.src/binaryen.path'. Skipping second one!
warning: e9a64996bedbc224d1edb401162d2c3cc15eb7c1:.gitmodules, multiple configurations found for 'submodule.src/binaryen.url'. Skipping second one!
warning: e9a64996bedbc224d1edb401162d2c3cc15eb7c1:.gitmodules, multiple configurations found for 'submodule.src/doc/rust-by-example.path'. Skipping second one!
warning: e9a64996bedbc224d1edb401162d2c3cc15eb7c1:.gitmodules, multiple configurations found for 'submodule.src/doc/rust-by-example.url'. Skipping second one!
Searching for toolstate changes between a72c360a30f9a8160e4f40340cecc9b1ce979cd7 and 9b6d317093d0766f33ae4aa8b47997556d9568be
Executing the job since rustdoc was updated
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
From https://github.com/rust-lang/miri
 * branch              062cf07d98822beb4c5b1132f47b4397275f403a -> FETCH_HEAD
 * branch              06758c48bd7a77bb5aa43fc50cf344540ba5afef -> FETCH_HEAD
 * branch              dd7f545a69e4b720407e458bf4ade0b207bbf9ee -> FETCH_HEAD
fatal: remote error: upload-pack: not our ref 9ad6e5b8f68ee4bcd85886e50b8b0a70cbb91a52
Errors during submodule fetch:
##[error]Process completed with exit code 1.
Post job cleanup.
