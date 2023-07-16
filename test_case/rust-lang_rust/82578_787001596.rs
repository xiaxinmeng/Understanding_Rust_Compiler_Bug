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
   Compiling addr2line v0.14.0
error: unused attribute
    --> library/std/src/path.rs:1077:23
     |
1077 | #[cfg_attr(not(test), rustc_diagnostic_item = "PathBuf")]
     |
     |
     = note: `-D unused-attributes` implied by `-D warnings`
error: unused attribute
    --> library/std/src/path.rs:1732:23
     |
     |
1732 | #[cfg_attr(not(test), rustc_diagnostic_item = "Path")]

error: aborting due to 2 previous errors

error: could not compile `std`
