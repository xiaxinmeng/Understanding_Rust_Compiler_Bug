plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between c2ecd3af87477147695aa3f6e1237e3185044e62 and df0cb1c9f07b432e0dfc34e9dea6ffaf97de065c
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (60/66)
......     (66/66)


/checkout/src/test/rustdoc-gui/item-info-width.goml item-info-width... FAILED
[ERROR] (line 6) Error: Evaluation failed: The following errors happened (for selector `.item-info`): [expected `790px` for key `width`, found `288.891px` (or `289px`)]: for command `assert-css: (".item-info", {"width": "790px"})`

/checkout/src/test/rustdoc-gui/item-info-overflow.goml item-info-overflow... FAILED
[ERROR] (line 6) Error: Evaluation failed: scrollWidth: `890` !== `668`: for command `compare-elements-property: (".docblock.item-decl", ".item-info", ["scrollWidth"])`
[ERROR] (line 7) Error: Evaluation failed: property `scrollWidth` (`668`) isn't equal to `890` for selector `.item-info`: for command `assert-property: (".item-info", {"scrollWidth": "890"})`
[ERROR] (line 17) Error: Evaluation failed: scrollWidth: `668` !== `866`: for command `compare-elements-property: (
    "#impl-SimpleTrait-for-LongItemInfo2 .item-info",
    "#impl-SimpleTrait-for-LongItemInfo2 + .docblock",
    ["scrollWidth"],
)`
[ERROR] (line 22) Error: Evaluation failed: property `scrollWidth` (`668`) isn't equal to `866` for selector `#impl-SimpleTrait-for-LongItemInfo2 .item-info`: for command `assert-property: (
    "#impl-SimpleTrait-for-LongItemInfo2 .item-info",
    {"scrollWidth": "866"},

Build completed unsuccessfully in 0:00:22
