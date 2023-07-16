plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between c6007fdc7059c677a6c089e8d2915b264c0d1326 and 360132449eee2e145d5005e04394dfda6e741108
Submodules were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2021-09-08/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
    Updating crates.io index
    Updating git repository `https://github.com/rust-lang/cargo`
error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
If you want to try to generate the lock file without accessing the network, remove the --locked flag and use --offline instead.
Build completed unsuccessfully in 0:00:41
make: *** [prepare] Error 1
Makefile:60: recipe for target 'prepare' failed
Command failed. Attempt 2/5:
Command failed. Attempt 2/5:
    Updating crates.io index
    Updating git repository `https://github.com/rust-lang/cargo`
error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
If you want to try to generate the lock file without accessing the network, remove the --locked flag and use --offline instead.
Build completed unsuccessfully in 0:00:00
make: *** [prepare] Error 1
Makefile:60: recipe for target 'prepare' failed
Command failed. Attempt 3/5:
Command failed. Attempt 3/5:
    Updating crates.io index
    Updating git repository `https://github.com/rust-lang/cargo`
error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
If you want to try to generate the lock file without accessing the network, remove the --locked flag and use --offline instead.
Build completed unsuccessfully in 0:00:00
make: *** [prepare] Error 1
Makefile:60: recipe for target 'prepare' failed
Command failed. Attempt 4/5:
Command failed. Attempt 4/5:
    Updating crates.io index
    Updating git repository `https://github.com/rust-lang/cargo`
error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
If you want to try to generate the lock file without accessing the network, remove the --locked flag and use --offline instead.
Build completed unsuccessfully in 0:00:00
make: *** [prepare] Error 1
Makefile:60: recipe for target 'prepare' failed
Command failed. Attempt 5/5:
Command failed. Attempt 5/5:
    Updating crates.io index
    Updating git repository `https://github.com/rust-lang/cargo`
error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
If you want to try to generate the lock file without accessing the network, remove the --locked flag and use --offline instead.
Build completed unsuccessfully in 0:00:00
make: *** [prepare] Error 1
Makefile:60: recipe for target 'prepare' failed
The command has failed after 5 attempts.
