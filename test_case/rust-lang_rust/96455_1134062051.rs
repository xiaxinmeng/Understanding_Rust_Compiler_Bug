plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 07e7b4346c48fe6d5c3026207e2d04c9a9f00fb8 and 39f628fd0c5bfedb296f3c086dfcd18d7868b4df
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (60/64)
...       (64/64)


/checkout/src/test/rustdoc-gui/escape-key.goml escape-key... FAILED
[ERROR] (line 6) Error: The following CSS selector "#search h1" was not found: for command `wait-for: "#search h1" // The search element is empty before the first search `
Build completed unsuccessfully in 0:00:41
