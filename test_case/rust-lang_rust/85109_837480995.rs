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
  |
1 | #![feature(const_fn)]
  |            ^^^^^^^^ feature has been removed
  |
  = note: split into finer-grained feature gates
error: aborting due to previous error

For more information about this error, try `rustc --explain E0557`.




The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestJTGPQn/calls.stage-id.stderr
To only update this specific test, also pass `--test-args calls.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/calls.rs" "-L" "/tmp/compiletestJTGPQn" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestJTGPQn/calls.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestJTGPQn/calls.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"feature has been removed","code":{"code":"E0557","explanation":"A feature attribute named a feature that has been removed.\n\nErroneous code example:\n\n