plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 3b98c08288528c243973a54934e4be75bcf20c31 and 7dee8a3828966b65c80e4f066e953180017a55e7
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (60/64)
...       (64/64)


/checkout/src/test/rustdoc-gui/headings.goml headings... FAILED
[ERROR] (line 109) ".sidebar .others h3" not found: for command `assert-text: (".sidebar .others h3", "Modules")`
[ERROR] (line 110) ".sidebar .others h3" not found: for command `assert-css: (".sidebar .others h3", {"border-bottom-width": "0px"}, ALL)`

/checkout/src/test/rustdoc-gui/duplicate-macro-reexport.goml duplicate-macro-reexport... FAILED
[ERROR] (line 4) Error: The following CSS selector ".sidebar-elems .others .macro" was not found: for command `wait-for: ".sidebar-elems .others .macro"`
Build completed unsuccessfully in 0:00:44
