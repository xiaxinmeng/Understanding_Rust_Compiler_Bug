plain
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  IMAGE: x86_64-gnu-llvm-12
##[endgroup]
fatal: unknown commit 53fd98ca776cb875bc9e5514f56b52eb74f9e7a9
All commits in `HEAD` are present in `master`
src/ci/scripts/verify-stable-version-number.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-12
---
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-06-30/cargo-1.62.0-x86_64-unknown-linux-gnu.tar.xz
Building rustbuild
    Updating crates.io index
error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
If you want to try to generate the lock file without accessing the network, remove the --locked flag and use --offline instead.
Build completed unsuccessfully in 0:00:41
make: *** [Makefile:58: prepare] Error 1
Command failed. Attempt 2/5:
Building rustbuild
Building rustbuild
    Updating crates.io index
error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
If you want to try to generate the lock file without accessing the network, remove the --locked flag and use --offline instead.
Build completed unsuccessfully in 0:00:00
make: *** [Makefile:58: prepare] Error 1
Command failed. Attempt 3/5:
Building rustbuild
Building rustbuild
    Updating crates.io index
error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
If you want to try to generate the lock file without accessing the network, remove the --locked flag and use --offline instead.
Build completed unsuccessfully in 0:00:00
make: *** [Makefile:58: prepare] Error 1
Command failed. Attempt 4/5:
Building rustbuild
Building rustbuild
    Updating crates.io index
error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
If you want to try to generate the lock file without accessing the network, remove the --locked flag and use --offline instead.
Build completed unsuccessfully in 0:00:00
make: *** [Makefile:58: prepare] Error 1
Command failed. Attempt 5/5:
Building rustbuild
Building rustbuild
    Updating crates.io index
error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
If you want to try to generate the lock file without accessing the network, remove the --locked flag and use --offline instead.
Build completed unsuccessfully in 0:00:00
make: *** [Makefile:58: prepare] Error 1
The command has failed after 5 attempts.
