plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between d6f99e535a301a421dfee52a7c25bb4bdf420344 and d7c49c5c803cf8ecc89f4c6bd82feea8c7022f39
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (90/97)
.......    (97/97)


/checkout/src/test/rustdoc-gui/basic-code.goml basic-code... FAILED
[ERROR] (line 3) Error: Execution context was destroyed, most likely because of a navigation.: for command `wait-for: ".src-line-numbers"`
Build completed unsuccessfully in 0:02:38
