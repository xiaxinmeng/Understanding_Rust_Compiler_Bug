plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 7f997f589f4e0b1c59a3680e7a8dd941d3ada518 and c3aacb66a69e3aea1713ac7a74a6383eec2d8138
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (60/64)
...       (64/64)


/checkout/src/test/rustdoc-gui/settings.goml settings... FAILED
[ERROR] (line 49) Error: The following CSS selector ".setting-line:not(.hidden) #preferred-dark-theme" was not found: for command `wait-for: ".setting-line:not(.hidden) #preferred-dark-theme"`
Build completed unsuccessfully in 0:00:46
