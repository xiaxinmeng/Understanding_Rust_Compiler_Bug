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
Searching for toolstate changes between 3198c523cc1f38f947bafa983f711cf68d8c4468 and c369891b5cea56e04d79f8b773f2027bf67b45a6
Executing the job since rustdoc was updated
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
DirectMap4k:      247744 kB
DirectMap2M:     4995072 kB
DirectMap1G:    55574528 kB
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
".git\n"
origin https://github.com/rust-lang/rust (fetch)
origin https://github.com/rust-lang/rust (push)
total 12
drwxr-xr-x 3 user 116 4096 Jun 19 13:58 pull
drwxr-xr-x 5 user 116 4096 Jun 19 13:58 ..
drwxr-xr-x 3 user 116 4096 Jun 19 13:58 .
drwxr-xr-x 3 user 116 4096 Jun 19 13:58 .

* (HEAD detached at pull/86413/merge)

error: cannot open .git/FETCH_HEAD: Read-only file system

thread 'main' panicked at 'command did not execute successfully: "git" "fetch" "origin" "master"
expected success, got: exit status: 255', src/build_helper/lib.rs:139:9
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 --no-fail-fast src/doc/book src/doc/nomicon src/doc/reference src/doc/rust-by-example src/doc/embedded-book src/doc/edition-guide src/tools/rls src/tools/miri
Build completed unsuccessfully in 0:00:01
cat: /tmp/toolstate/toolstates.json: No such file or directory
