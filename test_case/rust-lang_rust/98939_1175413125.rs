plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between efb171e2350de2bec6dd1f035b99bc00535c1c15 and b39d6ecc94da1592dddbec43e490e26aedbbaa77
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (60/66)
......     (66/66)


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
[ERROR] (line 19) "#impl-EmptyTrait1" not found: for command `compare-elements-position-near-false: ("#impl-EmptyTrait1", "#impl-EmptyTrait2", {"y": 30})`
[ERROR] (line 20) "#impl-EmptyTrait3 h3" not found: for command `compare-elements-position-near: ("#impl-EmptyTrait3 h3", "#impl-EmptyTrait3 .item-info", {"y": 30})`
Build completed unsuccessfully in 0:00:19
