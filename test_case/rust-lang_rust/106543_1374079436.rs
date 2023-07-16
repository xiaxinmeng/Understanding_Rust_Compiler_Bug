plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between afe8c4537c9009a251a31e8f022b7795fc305d4f and 6e4cfdde60de5cf1b47f749e11da7ff3556d0615
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (90/97)
.......    (97/97)


/checkout/src/test/rustdoc-gui/sidebar-source-code-display.goml sidebar-source-code-display... FAILED
[ERROR] (line 174) Error: The following CSS properties still don't match: [width: (`200px` != `0px`)]: for command `wait-for-css: (".sidebar", {"width": "0px"})`

/checkout/src/test/rustdoc-gui/sidebar-source-code.goml sidebar-source-code... FAILED
[ERROR] (line 84) Error: Evaluation failed: The following errors happened (for selector `nav.sidebar`): [expected `0px` for key `width`, found `200px`]: for command `assert-css: ("nav.sidebar", {"width": "0px"})`
Build completed unsuccessfully in 0:02:57
