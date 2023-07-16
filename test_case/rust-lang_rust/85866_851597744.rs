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
Searching for toolstate changes between d9feaaa548ce380159a1de68f4f6e605db9a9fc5 and e90af200d6ff31f9cc6ac116831671840c8c29a7
Executing the job since submodules are updated
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---

######################################################################## 100.0%
extracting /checkout/obj/build/cache/2021-03-25/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
    Updating crates.io index
error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
If you want to try to generate the lock file without accessing the network, use the --offline flag.
Build completed unsuccessfully in 0:00:34
make: *** [prepare] Error 1
Makefile:60: recipe for target 'prepare' failed
Command failed. Attempt 2/5:
Command failed. Attempt 2/5:
    Updating crates.io index
error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
If you want to try to generate the lock file without accessing the network, use the --offline flag.
Build completed unsuccessfully in 0:00:00
make: *** [prepare] Error 1
Makefile:60: recipe for target 'prepare' failed
Command failed. Attempt 3/5:
Command failed. Attempt 3/5:
    Updating crates.io index
error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
If you want to try to generate the lock file without accessing the network, use the --offline flag.
Build completed unsuccessfully in 0:00:00
make: *** [prepare] Error 1
Makefile:60: recipe for target 'prepare' failed
Command failed. Attempt 4/5:
Command failed. Attempt 4/5:
    Updating crates.io index
error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
If you want to try to generate the lock file without accessing the network, use the --offline flag.
Build completed unsuccessfully in 0:00:00
make: *** [prepare] Error 1
Makefile:60: recipe for target 'prepare' failed
Command failed. Attempt 5/5:
Command failed. Attempt 5/5:
    Updating crates.io index
error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
If you want to try to generate the lock file without accessing the network, use the --offline flag.
Build completed unsuccessfully in 0:00:00
make: *** [prepare] Error 1
Makefile:60: recipe for target 'prepare' failed
The command has failed after 5 attempts.
