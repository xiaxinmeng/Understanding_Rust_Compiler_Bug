plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 32c8c5df06c025441ad04791d7982d65c79a60e4 and 1b08604ce62bf28a818ae577e72a6080fee7df41
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
