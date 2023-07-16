plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 87788097b776f8e3662f76627944230684b671bd and d90f2ce7e9f4b3cda35813b18db0f21d6e4e4343
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (70/72)
..         (72/72)


/checkout/src/test/rustdoc-gui/code-tags.goml code-tags... FAILED
[ERROR] (line 6) expected 3 elements, found 4: for command `assert-count: (".example-wrap pre > code", 3)`
Build completed unsuccessfully in 0:01:37
