plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between b04c5329e1e145fb2fb46c5a7e775638712b03aa and 392a079b57009218a9ecccd7961f977171ca1e07
Submodules were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (60/62)
.         (62/62)


/checkout/src/test/rustdoc-gui/escape-key.goml escape-key... FAILED
[ERROR] (line 6) TimeoutError: waiting for selector "#search h1" failed: timeout 30000ms exceeded: for command `wait-for: "#search h1" // The search element is empty before the first search `
Build completed unsuccessfully in 0:00:41
