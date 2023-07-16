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
   Compiling libc v0.2.93
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0658]: the `#[no_coverage]` attribute is an experimental feature
    |
    |
277 |     #[no_coverage]
    |
    = note: see issue #84605 <https://github.com/rust-lang/rust/issues/84605> for more information
    = note: see issue #84605 <https://github.com/rust-lang/rust/issues/84605> for more information
    = help: add `#![feature(no_coverage)]` to the crate attributes to enable
    = help: or, alternatively, add `#[feature(no_coverage)]` to the function
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
error: could not compile `core`
