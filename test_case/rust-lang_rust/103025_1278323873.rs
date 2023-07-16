plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 6b3ede3f7bc502eba7bbd202b4b9312d812adcd7 and 4ba7ec295001d4fff341a42e3cdb5d7dc5db4533
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (70/78)
........   (78/78)


/checkout/src/test/rustdoc-gui/basic-code.goml basic-code... FAILED
[ERROR] (line 3) expected 1 elements, found 0: for command `assert-count: (".src-line-numbers", 1)`
Build completed unsuccessfully in 0:02:14
