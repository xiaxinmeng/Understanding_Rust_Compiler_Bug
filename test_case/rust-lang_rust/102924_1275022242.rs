plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between bb93450ec4af83c20e9ba6c8e575aca55423001a and 4a73ea9eb1a5f4f92c95ef179c68682be16ae54c
src/ci/scripts/should-skip-this.sh: line 23: library/std/src/sys: Is a directory
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (70/78)
........   (78/78)


/checkout/src/test/rustdoc-gui/basic-code.goml basic-code... FAILED
[ERROR] (line 3) expected 1 elements, found 0: for command `assert-count: (".src-line-numbers", 1)`
Build completed unsuccessfully in 0:02:18
