plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 0e21a27075a8c508f00d1a4430497f17ce93d5c9 and 67ce8b0ce2ae40f9f19eb8e0339f1f7816d2f2a4
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
Verifying status of edition-guide...
Verifying status of rls...
Verifying status of miri...
Verifying status of embedded-book...
fatal: could not create work tree dir 'rust-toolstate': Read-only file system
thread 'main' panicked at 'git clone unsuccessful (status: Ok(ExitStatus(unix_wait_status(32768))))', toolstate.rs:322:9
Build completed unsuccessfully in 0:00:00
