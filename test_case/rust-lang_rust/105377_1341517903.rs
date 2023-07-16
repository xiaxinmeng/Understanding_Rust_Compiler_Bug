plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between ec28f5338b8e54fa8ae3c18bf101c809c337f1f5 and 949df9ab240d29afbd08a66060d88e06ec3839b8
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
    Checking miniz_oxide v0.5.3
    Checking hashbrown v0.12.3
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking addr2line v0.17.0
error[E0425]: cannot find value `POSIX_SPAWN_SETSID` in crate `libc`
     |
     |
553  |                 flags |= libc::POSIX_SPAWN_SETSID;
     |                                ^^^^^^^^^^^^^^^^^^ help: a constant with a similar name exists: `POSIX_SPAWN_SETSIGDEF`
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.138/src/unix/bsd/apple/mod.rs:4555:1
     |
     |
4555 | pub const POSIX_SPAWN_SETSIGDEF: ::c_int = 0x04;
     | ---------------------------------------- similarly named constant `POSIX_SPAWN_SETSIGDEF` defined here
For more information about this error, try `rustc --explain E0425`.
error: could not compile `std` due to previous error
fatal error: failed to build sysroot, see error details above
Build completed unsuccessfully in 0:00:36
