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
   Compiling libc v0.2.79
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0635]: unknown feature `doc_notable_trait`
    |
    |
111 | #![feature(doc_notable_trait)]

error: aborting due to previous error

For more information about this error, try `rustc --explain E0635`.
