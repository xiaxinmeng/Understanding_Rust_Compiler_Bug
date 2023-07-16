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
Searching for toolstate changes between 8a57820bca64a252489790a57cb5ea23db6f9198 and e3b3870252db506c8e849c936dad7c9cd24e2b63
Executing the job since clippy or rustfmt subtree was updated
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error: the feature `never_type` has been stable since 1.54.0 and no longer requires an attribute to enable
  --> src/librustdoc/lib.rs:14:12
   |
14 | #![feature(never_type)]
   |
   |
   = note: `-D stable-features` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustdoc`

