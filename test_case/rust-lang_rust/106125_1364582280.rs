plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between f5c3dfdbbf06d5997079ac7339de5002f7ced2a3 and 52d1dfac2e7b9eb15d8adf7b0a2ddfeb1fff2501
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (90/95)
.....      (95/95)


/checkout/src/test/rustdoc-gui/notable-trait.goml notable-trait... FAILED
[ERROR] (line 48) Error: Evaluation failed: assert didn't fail: for command `compare-elements-position-false: (
    "//*[@id='method.create_an_iterator_from_read']//a[text()='NotableStructWithLongName']",
    "//*[@id='method.create_an_iterator_from_read']//*[@class='notable-traits']",
    ("y", "x"),
)`
[ERROR] (line 69) Error: Evaluation failed: The following errors happened: [different X values: 677.03125 (or 677) != 245]: for command `assert-position: (
    "//*[@id='method.create_an_iterator_from_read']//a[text()='NotableStructWithLongName']",
    {"x": 245},
)`
[ERROR] (line 73) Error: Evaluation failed: The following errors happened: [different X values: 955.453125 (or 955) != 523]: for command `assert-position: (
    "//*[@id='method.create_an_iterator_from_read']//*[@class='notable-traits']",
    {"x": 523},
)`
[ERROR] (line 93) Error: Evaluation failed: The following errors happened: [different X values: 447.03125 (or 447) != 15]: for command `assert-position: (
    "//*[@id='method.create_an_iterator_from_read']//a[text()='NotableStructWithLongName']",
    {"x": 15},
)`
[ERROR] (line 97) Error: Evaluation failed: The following errors happened: [different X values: 725.453125 (or 725) != 293]: for command `assert-position: (
    "//*[@id='method.create_an_iterator_from_read']//*[@class='notable-traits']",
    {"x": 293},
)`
[ERROR] (line 102) Error: Node is either not clickable or not an HTMLElement: for command `click: "//*[@id='method.create_an_iterator_from_read']//*[@class='notable-traits']"`
Build completed unsuccessfully in 0:02:31
