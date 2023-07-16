plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between bb8c2f41174caceec00c28bc6c5c20ae9f9a175c and 86d81aa2c0ee8fe7cdffa1320f66f9c34f733aab
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (60/64)
...       (64/64)


/checkout/src/test/rustdoc-gui/duplicate-macro-reexport.goml duplicate-macro-reexport... FAILED
[ERROR] (line 4) Error: The following CSS selector ".sidebar-elems .others .macro" was not found: for command `wait-for: ".sidebar-elems .others .macro"`
Build completed unsuccessfully in 0:00:42
