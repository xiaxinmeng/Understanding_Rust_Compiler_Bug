plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between a474ebbc4e975683a296739bd9b11b5dde83f808 and 443e3ca439fb287cbddd4abd83a2c033627c09f1
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (90/93)
...        (93/93)


/checkout/src/test/rustdoc-gui/basic-code.goml basic-code... FAILED
[ERROR] (line 3) Error: Execution context was destroyed, most likely because of a navigation.: for command `assert-count: (".src-line-numbers", 1)`
Build completed unsuccessfully in 0:02:30
