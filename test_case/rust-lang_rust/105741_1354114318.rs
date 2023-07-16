plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between d67000e44e1b9908c81fc4d5de875608f1b80ae9 and 0f06c4429af3fc91e76b35a17be871a86ebf3cbd
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
33 | pub(crate) macro weak {
   | --------------------- previous definition of the macro `weak` here
...
47 | pub(crate) macro weak {
   | ^^^^^^^^^^^^^^^^^^^^^ `weak` redefined here
   |
   = note: `weak` must be defined only once in the macro namespace of this module
For more information about this error, try `rustc --explain E0428`.
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:00:21
cat: /tmp/toolstate/toolstates.json: No such file or directory
