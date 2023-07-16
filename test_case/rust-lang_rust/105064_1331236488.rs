plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between e0098a5cc3a87d857e597af824d0ce1ed1ad85e0 and bda59de75c6039c158a5f0b57ff80f597fb1bb6c
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
......... (80/90)
......... (90/90)


/checkout/src/test/rustdoc-gui/code-blocks-overflow.goml code-blocks-overflow... FAILED
[ERROR] (line 8) Error: Evaluation failed: The following errors happened (for selector `.docblock > .example-wrap > pre`): [expected `796px` for key `width`, found `936px`]: for command `assert-css: (".docblock > .example-wrap > pre", {"width": "796px", "overflow-x": "auto"}, ALL)`

/checkout/src/test/rustdoc-gui/docblock-table-overflow.goml docblock-table-overflow... FAILED
[ERROR] (line 7) Error: Evaluation failed: The following errors happened (for selector `.top-doc .docblock`): [Expected `816` for property `scrollWidth`, found `936`]: for command `assert-property: (".top-doc .docblock", {"scrollWidth": "816"})`
[ERROR] (line 19) Error: Evaluation failed: The following errors happened (for selector `#implementations-list > details .docblock`): [Expected `816` for property `scrollWidth`, found `936`]: for command `assert-property: ("#implementations-list > details .docblock", {"scrollWidth": "816"})`

/checkout/src/test/rustdoc-gui/search-result-display.goml search-result-display... FAILED
[ERROR] (line 35) Error: Evaluation failed: The following errors happened (for selector `#crate-search`): [expected `527px` for key `width`, found `760px`]: for command `assert-css: ("#crate-search", {"width": "527px"})`
[ERROR] (line 36) Error: Evaluation failed: The following errors happened (for selector `.search-results-title`): [expected `640px` for key `width`, found `873.172px` (or `873px`)]: for command `assert-css: (".search-results-title", {"height": "44px", "width": "640px"})`
[ERROR] (line 38) Error: Evaluation failed: The following errors happened (for selector `#search`): [expected `640px` for key `width`, found `873.172px` (or `873px`)]: for command `assert-css: ("#search", {"width": "640px"})`

/checkout/src/test/rustdoc-gui/sidebar-source-code-display.goml sidebar-source-code-display... FAILED
[ERROR] (line 21 from 122) "#sidebar-toggle > button:hover" not found: for command `assert-css: (
            "#sidebar-toggle > button:hover",
            {"background-color": |background_toggle_hover|},
        )`
[ERROR] (line 40 from 122) "#source-sidebar details[open] > .files a:not(.selected):hover" not found: for command `assert-css: (
            "#source-sidebar details[open] > .files a:not(.selected):hover",
            {"color": |color_hover|, "background-color": |background_hover|},
        )`
[ERROR] (line 59 from 122) "#source-sidebar .dir-entry summary:hover" not found: for command `assert-css: (
            "#source-sidebar .dir-entry summary:hover",
            {"color": |color_hover|, "background-color": |background_hover|},
        )`
[ERROR] (line 78 from 122) "#source-sidebar details[open] > .folders > details > summary:hover" not found: for command `assert-css: (
            "#source-sidebar details[open] > .folders > details > summary:hover",
            {"color": |color_hover|, "background-color": |background_hover|},
        )`
[ERROR] (line 21 from 131) "#sidebar-toggle > button:hover" not found: for command `assert-css: (
            "#sidebar-toggle > button:hover",
            {"background-color": |background_toggle_hover|},
        )`
[ERROR] (line 40 from 131) "#source-sidebar details[open] > .files a:not(.selected):hover" not found: for command `assert-css: (
            "#source-sidebar details[open] > .files a:not(.selected):hover",
            {"color": |color_hover|, "background-color": |background_hover|},
        )`
[ERROR] (line 59 from 131) "#source-sidebar .dir-entry summary:hover" not found: for command `assert-css: (
            "#source-sidebar .dir-entry summary:hover",
            {"color": |color_hover|, "background-color": |background_hover|},
        )`
[ERROR] (line 78 from 131) "#source-sidebar details[open] > .folders > details > summary:hover" not found: for command `assert-css: (
            "#source-sidebar details[open] > .folders > details > summary:hover",
            {"color": |color_hover|, "background-color": |background_hover|},
        )`
[ERROR] (line 21 from 140) "#sidebar-toggle > button:hover" not found: for command `assert-css: (
            "#sidebar-toggle > button:hover",
            {"background-color": |background_toggle_hover|},
        )`
[ERROR] (line 40 from 140) "#source-sidebar details[open] > .files a:not(.selected):hover" not found: for command `assert-css: (
            "#source-sidebar details[open] > .files a:not(.selected):hover",
            {"color": |color_hover|, "background-color": |background_hover|},
        )`
[ERROR] (line 59 from 140) "#source-sidebar .dir-entry summary:hover" not found: for command `assert-css: (
            "#source-sidebar .dir-entry summary:hover",
            {"color": |color_hover|, "background-color": |background_hover|},
        )`
[ERROR] (line 78 from 140) "#source-sidebar details[open] > .folders > details > summary:hover" not found: for command `assert-css: (
            "#source-sidebar details[open] > .folders > details > summary:hover",
            {"color": |color_hover|, "background-color": |background_hover|},
        )`

/checkout/src/test/rustdoc-gui/type-declation-overflow.goml type-declation-overflow... FAILED
[ERROR] (line 12) Error: Evaluation failed: The following errors happened (for selector `body`): [Expected `1100` for property `scrollWidth`, found `1220`]: for command `assert-property: ("body", {"scrollWidth": "1100"})`
[ERROR] (line 22) Error: Evaluation failed: The following errors happened (for selector `body`): [Expected `1100` for property `scrollWidth`, found `1220`]: for command `assert-property: ("body", {"scrollWidth": "1100"})`
[ERROR] (line 24) Error: Evaluation failed: The following errors happened (for selector `#main-content`): [Expected `840` for property `scrollWidth`, found `960`]: for command `assert-property: ("#main-content", {"scrollWidth": "840"})`
[ERROR] (line 31) Error: Evaluation failed: The following errors happened (for selector `body`): [Expected `1100` for property `scrollWidth`, found `1210`]: for command `assert-property: ("body", {"scrollWidth": "1100"})`
[ERROR] (line 33) Error: Evaluation failed: The following errors happened (for selector `#main-content`): [Expected `840` for property `scrollWidth`, found `950`]: for command `assert-property: ("#main-content", {"scrollWidth": "840"})`
Build completed unsuccessfully in 0:02:18
