plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 8681d4cffcd23bbe619984ab62772a91827a40dc and 0f11ffb90a374db29f891a480c6f03ecf9001773
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (80/90)
.......... (90/90)


/checkout/src/test/rustdoc-gui/basic-code.goml basic-code... FAILED
[ERROR] (line 3) Error: Execution context was destroyed, most likely because of a navigation.: for command `assert-count: (".src-line-numbers", 1)`
Build completed unsuccessfully in 0:02:24
