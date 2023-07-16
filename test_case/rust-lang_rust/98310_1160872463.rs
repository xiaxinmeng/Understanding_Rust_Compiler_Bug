plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between b12708f7f40463b2131f0c47d1e8a4ffb543a422 and 203c8159ea59c0c4fa2f8a5aad01b7173f579e59
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
......... (60/64)
...       (64/64)


/checkout/src/test/rustdoc-gui/sidebar-source-code-display.goml sidebar-source-code-display... FAILED
[ERROR] (line 15) Error: Evaluation failed: expected `hidden` for key `visibility` for selector `.sidebar > *:not(#sidebar-toggle)`, found `visible`: for command `assert-css: (".sidebar > *:not(#sidebar-toggle)", {"visibility": "hidden", "opacity": 0})`

/checkout/src/test/rustdoc-gui/source-anchor-scroll.goml source-anchor-scroll... FAILED
[ERROR] (line 11) Error: Evaluation failed: property `scrollTop` (`0`) isn't equal to `125` for selector `html`: for command `assert-property: ("html", {"scrollTop": "125"})`
[ERROR] (line 13) Error: Evaluation failed: property `scrollTop` (`0`) isn't equal to `166` for selector `html`: for command `assert-property: ("html", {"scrollTop": "166"})`
[ERROR] (line 15) Error: Evaluation failed: property `scrollTop` (`166`) isn't equal to `53` for selector `html`: for command `assert-property: ("html", {"scrollTop": "53"})`
[ERROR] (line 20) Error: Evaluation failed: property `scrollTop` (`166`) isn't equal to `53` for selector `html`: for command `assert-property: ("html", {"scrollTop": "53"})`

/checkout/src/test/rustdoc-gui/source-code-page.goml source-code-page... FAILED
[ERROR] (line 26) Error: Evaluation failed: different X values: 354.203125(or 354) != 104: for command `assert-position: ("//*[@id='1']", {"x": 104, "y": 103})`
[ERROR] (line 35) ".sidebar.expanded" not found: for command `assert: ".sidebar.expanded"`
[ERROR] (line 39) Error: Evaluation failed: attribute `class` (`name expand`) isn't equal to `name` for selector `#source-sidebar .name:nth-child(2)`: for command `assert-attribute: ("#source-sidebar .name:nth-child(2)", {"class": "name"})`
[ERROR] (line 42) Error: Evaluation failed: expected `none` for key `display` for selector `#source-sidebar .name:nth-child(2) + .children`, found `block`: for command `assert-css: ("#source-sidebar .name:nth-child(2) + .children", {"display": "none"})`
[ERROR] (line 45) Error: Evaluation failed: attribute `class` (`name`) isn't equal to `name expand` for selector `#source-sidebar .name:nth-child(2)`: for command `assert-attribute: ("#source-sidebar .name:nth-child(2)", {"class": "name expand"})`
[ERROR] (line 47) Error: Evaluation failed: expected `block` for key `display` for selector `#source-sidebar .name:nth-child(2) + .children`, found `none`: for command `assert-css: ("#source-sidebar .name:nth-child(2) + .children", {"display": "block"})`
[ERROR] (line 51) Error: Evaluation failed: attribute `class` (`name expand`) isn't equal to `name` for selector `#source-sidebar .name:nth-child(2)`: for command `assert-attribute: ("#source-sidebar .name:nth-child(2)", {"class": "name"})`
[ERROR] (line 52) Error: Evaluation failed: expected `none` for key `display` for selector `#source-sidebar .name:nth-child(2) + .children`, found `block`: for command `assert-css: ("#source-sidebar .name:nth-child(2) + .children", {"display": "none"})`

/checkout/src/test/rustdoc-gui/sidebar-source-code.goml sidebar-source-code... FAILED
[ERROR] (line 7) Error: Evaluation failed: expected `50px` for key `width` for selector `nav.sidebar`, found `300px`: for command `assert-css: ("nav.sidebar", {"width": "50px"})`
[ERROR] (line 11) Error: The following CSS selector "nav.sidebar.expanded" was not found: for command `wait-for-css: ("nav.sidebar.expanded", {"width": "300px"})`
Build completed unsuccessfully in 0:00:44
