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
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CI_ONLY_WHEN_SUBMODULES_CHANGED: 1
  IMAGE: x86_64-gnu-tools
##[endgroup]
Executing the job since clippy subtree was updated
with:
  github_token: ***
  check_every_seconds: 60
env:
---
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 2 tests
test check_that_clippy_has_the_same_major_version_as_rustc ... FAILED

failures:


---- check_that_clippy_has_the_same_major_version_as_rustc stdout ----
thread 'check_that_clippy_has_the_same_major_version_as_rustc' panicked at 'failed to run `rustc --version`: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/tools/clippy/tests/versioncheck.rs:37:14


failures:
failures:
    check_that_clippy_has_the_same_major_version_as_rustc
test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s

error: test failed, to rerun pass '--test versioncheck'

