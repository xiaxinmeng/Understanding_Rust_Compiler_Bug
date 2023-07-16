plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 4ecfdfac51b159f68fce608792affb34a70e6f73 and 45c55d41ac9bd6f732a95caa1e12b634dad653d5
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (70/74)
....       (74/74)


/checkout/src/test/rustdoc-gui/notable-trait.goml notable-trait... FAILED
[ERROR] (line 87) Error: Evaluation failed: assert didn't fail: for command `compare-elements-position-false: (
    "//*[@id='method.create_an_iterator_from_read']//a[text()='NotableStructWithLongName']",
    "//*[@id='method.create_an_iterator_from_read']//*[@class='notable-traits']",
    ("y", "x"),

Build completed unsuccessfully in 0:01:58
