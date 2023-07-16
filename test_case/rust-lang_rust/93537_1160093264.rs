plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 9a0b7749665d925d8f533756149deba74f2db88b and 989591fc3ee7232cf048b3ae1b1714b596b208c0
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (60/64)
....       (64/64)


/checkout/src/test/rustdoc-gui/item-info-overflow.goml item-info-overflow... FAILED
[ERROR] (line 17) "#impl-SimpleTrait .item-info" not found: for command `compare-elements-property: (
    "#impl-SimpleTrait .item-info",
    "#impl-SimpleTrait + .docblock",
    ["scrollWidth"],
)`
[ERROR] (line 22) "#impl-SimpleTrait .item-info" not found: for command `assert-property: ("#impl-SimpleTrait .item-info", {"scrollWidth": "866"})`
[ERROR] (line 24) "#impl-SimpleTrait .item-info" not found: for command `assert-text: (
    "#impl-SimpleTrait .item-info",
    "Available on Android or Linux or Emscripten or DragonFly BSD",
    STARTS_WITH,


/checkout/src/test/rustdoc-gui/implementors.goml implementors... FAILED
[ERROR] (line 20) "#impl-EmptyTrait3 .item-info" not found: for command `compare-elements-position-near: ("#impl-EmptyTrait3 h3", "#impl-EmptyTrait3 .item-info", {"y": 30})`
Build completed unsuccessfully in 0:00:18
