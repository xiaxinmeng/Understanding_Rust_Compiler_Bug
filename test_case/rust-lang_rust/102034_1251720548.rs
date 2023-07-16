plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 2019147c5642c08cdb9ad4cacd97dd1fa4ffa701 and 371afbdd82888a51faa501820f4d31220116a9db
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (70/73)
...        (73/73)


/checkout/src/test/rustdoc-gui/docblock-details.goml docblock-details... FAILED
[ERROR] (line 15) Error: Evaluation failed: The following errors happened (for selector `.top-doc .docblock summary h4`): [expected `0px none rgb(210, 210, 210)` for key `border-bottom`, found `0px none rgb(221, 221, 221)`]: for command `assert-css: (
    ".top-doc .docblock summary h4",
    {"border-bottom": "0px none rgb(210, 210, 210)"},


/checkout/src/test/rustdoc-gui/headings.goml headings... FAILED
[ERROR] (line 169) Error: Evaluation failed: The following errors happened (for selector `.top-doc .docblock h5`): [expected `0px none rgb(221, 221, 221)` for key `border-bottom`, found `0px none rgb(0, 0, 0)`]: for command `assert-css: (
    ".top-doc .docblock h5",
    {"color": "rgb(0, 0, 0)", "border-bottom": "0px none rgb(221, 221, 221)"},
)`
[ERROR] (line 173) Error: Evaluation failed: The following errors happened (for selector `#implementations-list .docblock h4`): [expected `0px none rgb(221, 221, 221)` for key `border-bottom`, found `0px none rgb(0, 0, 0)`]: for command `assert-css: (
    "#implementations-list .docblock h4",
    {"color": "rgb(0, 0, 0)", "border-bottom": "0px none rgb(221, 221, 221)"},
)`
[ERROR] (line 177) Error: Evaluation failed: The following errors happened (for selector `#implementations-list .docblock h5`): [expected `0px none rgb(221, 221, 221)` for key `border-bottom`, found `0px none rgb(0, 0, 0)`]: for command `assert-css: (
    "#implementations-list .docblock h5",
    {"color": "rgb(0, 0, 0)", "border-bottom": "0px none rgb(221, 221, 221)"},
)`
[ERROR] (line 181) Error: Evaluation failed: The following errors happened (for selector `#implementations-list .docblock h6`): [expected `0px none rgb(221, 221, 221)` for key `border-bottom`, found `0px none rgb(0, 0, 0)`]: for command `assert-css: (
    "#implementations-list .docblock h6",
    {"color": "rgb(0, 0, 0)", "border-bottom": "0px none rgb(221, 221, 221)"},
)`
[ERROR] (line 200) Error: Evaluation failed: The following errors happened (for selector `.top-doc .docblock h5`): [expected `0px none rgb(210, 210, 210)` for key `border-bottom`, found `0px none rgb(221, 221, 221)`]: for command `assert-css: (
    ".top-doc .docblock h5",
    {"color": "rgb(221, 221, 221)", "border-bottom": "0px none rgb(210, 210, 210)"},
)`
[ERROR] (line 204) Error: Evaluation failed: The following errors happened (for selector `#implementations-list .docblock h4`): [expected `0px none rgb(210, 210, 210)` for key `border-bottom`, found `0px none rgb(221, 221, 221)`]: for command `assert-css: (
    "#implementations-list .docblock h4",
    {"color": "rgb(221, 221, 221)", "border-bottom": "0px none rgb(210, 210, 210)"},
)`
[ERROR] (line 208) Error: Evaluation failed: The following errors happened (for selector `#implementations-list .docblock h5`): [expected `0px none rgb(210, 210, 210)` for key `border-bottom`, found `0px none rgb(221, 221, 221)`]: for command `assert-css: (
    "#implementations-list .docblock h5",
    {"color": "rgb(221, 221, 221)", "border-bottom": "0px none rgb(210, 210, 210)"},
)`
[ERROR] (line 212) Error: Evaluation failed: The following errors happened (for selector `#implementations-list .docblock h6`): [expected `0px none rgb(210, 210, 210)` for key `border-bottom`, found `0px none rgb(221, 221, 221)`]: for command `assert-css: (
    "#implementations-list .docblock h6",
    {"color": "rgb(221, 221, 221)", "border-bottom": "0px none rgb(210, 210, 210)"},
)`
[ERROR] (line 231) Error: Evaluation failed: The following errors happened (for selector `.top-doc .docblock h5`): [expected `0px none rgb(92, 103, 115)` for key `border-bottom`, found `0px none rgb(197, 197, 197)`]: for command `assert-css: (
    ".top-doc .docblock h5",
    {"color": "rgb(197, 197, 197)", "border-bottom": "0px none rgb(92, 103, 115)"},
)`
[ERROR] (line 235) Error: Evaluation failed: The following errors happened (for selector `#implementations-list .docblock h4`): [expected `0px none rgb(92, 103, 115)` for key `border-bottom`, found `0px none rgb(255, 255, 255)`]: for command `assert-css: (
    "#implementations-list .docblock h4",
    {"color": "rgb(255, 255, 255)", "border-bottom": "0px none rgb(92, 103, 115)"},
)`
[ERROR] (line 239) Error: Evaluation failed: The following errors happened (for selector `#implementations-list .docblock h5`): [expected `0px none rgb(92, 103, 115)` for key `border-bottom`, found `0px none rgb(197, 197, 197)`]: for command `assert-css: (
    "#implementations-list .docblock h5",
    {"color": "rgb(197, 197, 197)", "border-bottom": "0px none rgb(92, 103, 115)"},
)`
[ERROR] (line 243) Error: Evaluation failed: The following errors happened (for selector `#implementations-list .docblock h6`): [expected `0px none rgb(92, 103, 115)` for key `border-bottom`, found `0px none rgb(197, 197, 197)`]: for command `assert-css: (
    "#implementations-list .docblock h6",
    {"color": "rgb(197, 197, 197)", "border-bottom": "0px none rgb(92, 103, 115)"},


/checkout/src/test/rustdoc-gui/sidebar-mobile-scroll.goml sidebar-mobile-scroll... FAILED
[ERROR] (line 9) Error: Evaluation failed: The following errors happened: [Expected `643` for property `pageYOffset`, found `644`]: for command `assert-window-property: {"pageYOffset": "643"}`
[ERROR] (line 24) Error: Evaluation failed: The following errors happened: [Expected `643` for property `pageYOffset`, found `644`]: for command `assert-window-property: {"pageYOffset": "643"}`
[ERROR] (line 31) Error: Evaluation failed: The following errors happened: [Expected `643` for property `pageYOffset`, found `644`]: for command `assert-window-property: {"pageYOffset": "643"}`
Build completed unsuccessfully in 0:01:47
