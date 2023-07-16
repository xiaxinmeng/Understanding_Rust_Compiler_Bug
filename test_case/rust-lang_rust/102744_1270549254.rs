plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 2d46584fae1acc74566bf49fce976fe509a38f5f and 37bb2b5afe301fd30477d205c7139ba9f9f6a829
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (70/77)
.......    (77/77)


/checkout/src/test/rustdoc-gui/basic-code.goml basic-code... FAILED
[ERROR] (line 3) expected 1 elements, found 0: for command `assert-count: (".src-line-numbers", 1)`
Build completed unsuccessfully in 0:02:16
