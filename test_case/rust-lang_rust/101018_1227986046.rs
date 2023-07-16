plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between cfb5ae26a4496b7d35180f15e47ada0f3897c7e8 and 9502c876b085ef5f921f59b22b74371b4505e589
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (60/69)
.........  (69/69)


/checkout/src/test/rustdoc-gui/item-summary-table.goml item-summary-table... FAILED
[ERROR] (line 6) ".item-table .item-right" not found: for command `assert-text: (".item-table .item-right", "")`
Build completed unsuccessfully in 0:00:20
