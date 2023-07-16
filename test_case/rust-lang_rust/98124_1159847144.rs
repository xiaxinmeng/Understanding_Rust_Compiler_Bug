plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between bb8c2f41174caceec00c28bc6c5c20ae9f9a175c and 6bdeff70db6fa8b0089be79b81705ae192f053ab
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (60/64)
..       (64/64)


/checkout/src/test/rustdoc-gui/duplicate-macro-reexport.goml duplicate-macro-reexport... FAILED
[ERROR] (line 4) Error: The following CSS selector ".sidebar-elems .others .macro" was not found: for command `wait-for: ".sidebar-elems .others .macro"`

/checkout/src/test/rustdoc-gui/search-tab-change-title-fn-sig.goml search-tab-change-title-fn-sig... FAILED
[ERROR] (line 6) Error: The following CSS selector "#titles" was not found: for command `wait-for: "#titles"`
Build completed unsuccessfully in 0:00:44
