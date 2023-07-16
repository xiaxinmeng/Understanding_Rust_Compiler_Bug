plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 8d6f527530f4ba974d922269267fe89050188789 and 3665a43b69c411c3b4189a21f2a71967bba718b3
Submodules were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (50/60)
F (60/60)
.........

/checkout/src/test/rustdoc-gui/escape-key.goml escape-key... FAILED
[ERROR] (line 6) TimeoutError: waiting for selector "#search h1" failed: timeout 30000ms exceeded: for command `wait-for: "#search h1" // The search element is empty before the first search `
Build completed unsuccessfully in 0:00:44
