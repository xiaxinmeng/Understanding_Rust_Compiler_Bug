plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 7632db0e87d8adccc9a83a47795c9411b1455855 and 7d161cafab36d29ced844ba1f0ff7d363b0609a7
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (90/91)
.          (91/91)


/checkout/src/test/rustdoc-gui/sidebar.goml sidebar... FAILED
[ERROR] (line 91) ".sidebar-elems section .block li > a" not found: for command `assert-property: (".sidebar-elems section .block li > a", {"offsetHeight": 29})`
Build completed unsuccessfully in 0:02:04
