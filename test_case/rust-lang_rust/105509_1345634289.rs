plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between a161a7b654083a881b22908a475988bcc3221a79 and 3736c4596d07c7a54bd53550527ec561a62f764d
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (90/92)
..         (92/92)


/checkout/src/test/rustdoc-gui/stab-badge.goml stab-badge... FAILED
[ERROR] (line 6) Error: Evaluation failed: The following errors happened (for XPath `//*[@class='item-table']//*[@class='item-left module-item']/*[@class='stab deprecated']`): [expected `2px` for key `padding-top`, found `0px`]: for command `assert-css: (
    "//*[@class='item-table']//*[@class='item-left module-item']/*[@class='stab deprecated']",
    {"padding-top": "2px"},
)`
[ERROR] (line 10) Error: Evaluation failed: The following errors happened: [different Y values: 1891.390625 (or 1891) != 1892]: for command `assert-position: (
    "//*[@class='item-table']//*[@class='item-left module-item']/*[@class='stab deprecated']",
    {"y": 1892},
)`
[ERROR] (line 14) Error: Evaluation failed: The following errors happened: [different Y values: 1891.890625 (or 1892) != 1894]: for command `assert-position: (
    "//*[@class='item-table']//*[@class='item-left module-item']/*[@class='stab deprecated']/preceding-sibling::a",
    {"y": 1894}, // 1892 + 2 because of padding

Build completed unsuccessfully in 0:02:04
