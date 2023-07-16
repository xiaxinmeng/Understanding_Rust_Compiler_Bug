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
Searching for toolstate changes between a31431fce770ff90a347fd6114ac294e4568cbd8 and 4f5937afbcf584c657e03884b962c1624d28b498
Executing the job since rustdoc was updated
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---



The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestqQ9YIS/track-caller-attribute.stage-id.stderr
To only update this specific test, also pass `--test-args track-caller-attribute.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/track-caller-attribute.rs" "-L" "/tmp/compiletestqQ9YIS" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestqQ9YIS/track-caller-attribute.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestqQ9YIS/track-caller-attribute.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building rustdoc for stage2 (x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.20s
error: failed to write /checkout/src/test/rustdoc-gui/src/Cargo.lock
Caused by:
  failed to open: /checkout/src/test/rustdoc-gui/src/Cargo.lock

Caused by:
Caused by:
  Read-only file system (os error 30)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--workspace" "--target-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:04
