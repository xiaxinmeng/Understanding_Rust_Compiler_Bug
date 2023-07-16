plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between ead49f0beb7e36007aeed59f862f10f72b889c59 and cc84daaebc6456d8109cddd3b837fa4d138e679c
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
tests/pass/available-parallelism.rs ... ok
tests/pass/associated-const.rs ... ok
tests/pass/bools.rs ... ok
tests/pass/assume_bug.rs ... ok
tests/pass/available-parallelism-miri-num-cpus.rs ... ok
tests/pass/align.rs ... ok
tests/pass/atomic-compare-exchange-weak-never-fail.rs ... ok
tests/pass/async-fn.rs ... ok
tests/pass/arrays.rs ... ok
---
.......... (70/77)
.......    (77/77)


/checkout/src/test/rustdoc-gui/basic-code.goml basic-code... FAILED
[ERROR] (line 3) expected 1 elements, found 0: for command `assert-count: (".src-line-numbers", 1)`
Build completed unsuccessfully in 0:02:17
