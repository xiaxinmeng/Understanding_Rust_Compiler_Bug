plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 2d3a85b4f8ba7e2554f4d4fee126bc2ac6ee2af4 and f1b70aca84e327f1ab76166f6ee46fa14eda3a03
src/ci/scripts/should-skip-this.sh: line 23: library/std/src/sys: Is a directory
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (70/77)
.......    (77/77)


/checkout/src/test/rustdoc-gui/basic-code.goml basic-code... FAILED
[ERROR] (line 3) expected 1 elements, found 0: for command `assert-count: (".src-line-numbers", 1)`
Build completed unsuccessfully in 0:02:20
