plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between f5418b09e84883c4de2e652a147ab9faff4eee29 and 5c4efbfe1d9c4f7f315116bb5ea7dbb3b760e180
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (90/91)
.          (91/91)


/checkout/src/test/rustdoc-gui/basic-code.goml basic-code... FAILED
[ERROR] (line 3) Error: Execution context was destroyed, most likely because of a navigation.: for command `assert-count: (".src-line-numbers", 1)`
Build completed unsuccessfully in 0:02:25
