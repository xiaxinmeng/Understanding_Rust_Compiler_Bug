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
test [compile-fail] compile-fail/data_race/relax_acquire_race.rs ... ok

error: failure produced the wrong error: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/erroneous_const.rs" "-L" "/tmp/compiletestuuwj8J" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestuuwj8J/erroneous_const.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmir-opt-level=0" "-L" "/tmp/compiletestuuwj8J/erroneous_const.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"any use of this value will cause an error","code":{"code":"E0080","explanation":"A constant value failed to get evaluated.\n\nErroneous code example:\n\n