plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between b28d30e1e3c2b90fd08b7dd79d8e63884d1e0339 and bcaebb23592ecd8bbbf9befd5ca5c8b5b24f5a0d
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   |
85 |         use libc::open64;
   |             ^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: could not compile `std` due to previous error
fatal error: failed to build sysroot, see error details above
Build completed unsuccessfully in 0:01:49
