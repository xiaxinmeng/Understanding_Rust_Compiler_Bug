plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 760237ff785fd14ac7fdab799f4d695d86cf9cbf and f4a63fdddb742aa81b6e4bcf9a2bb0a0f12b3ae4
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (60/64)
...       (64/64)


/checkout/src/test/rustdoc-gui/escape-key.goml escape-key... FAILED
[ERROR] (line 6) Error: The following CSS selector "#search h1" was not found: for command `wait-for: "#search h1" // The search element is empty before the first search `
Build completed unsuccessfully in 0:00:44
