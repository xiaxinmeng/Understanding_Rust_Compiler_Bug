plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 11bb80a92b4f46fa7dfa9148d0bdfc185a7621bd and 2ca37075bd5bb38cffd640414191bd645fda62dc
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (70/73)
...        (73/73)


/checkout/src/test/rustdoc-gui/check-code-blocks-margin.goml check-code-blocks-margin... FAILED
[ERROR] (line 4) "#main-content .docblock.item-decl" not found: for command `assert-css: ("#main-content .docblock.item-decl", {"margin-left": "0px"})`

/checkout/src/test/rustdoc-gui/item-info-overflow.goml item-info-overflow... FAILED
[ERROR] (line 6) ".docblock.item-decl" not found: for command `compare-elements-property: (".docblock.item-decl", ".item-info", ["scrollWidth"])`
Build completed unsuccessfully in 0:01:58
