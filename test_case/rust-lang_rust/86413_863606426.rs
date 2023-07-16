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
 * branch              beta       -> FETCH_HEAD
Searching for toolstate changes between 3198c523cc1f38f947bafa983f711cf68d8c4468 and 69d2b22eb1d15061c5e979ecf4251371d59d051f
Executing the job since rustdoc was updated
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
DirectMap2M:     4988928 kB
DirectMap1G:    55574528 kB
+ /tmp/checktools.sh ../x.py
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
"origin\thttps://github.com/rust-lang/rust (fetch)\norigin\thttps://github.com/rust-lang/rust (push)\n"
ls: cannot access '.git/refs/remotes/origin': No such file or directory
thread 'main' panicked at 'command did not execute successfully: "ls" "-trla" ".git/refs/remotes/origin"
expected success, got: exit status: 2', src/build_helper/lib.rs:139:9
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 --no-fail-fast src/doc/book src/doc/nomicon src/doc/reference src/doc/rust-by-example src/doc/embedded-book src/doc/edition-guide src/tools/rls src/tools/miri
Build completed unsuccessfully in 0:00:01
cat: /tmp/toolstate/toolstates.json: No such file or directory
