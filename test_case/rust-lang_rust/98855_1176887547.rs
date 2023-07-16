plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 7665c3543079ebc3710b676d0fd6951bedfd4b29 and c2e979b7221877cdecaf03ea028dfcbacc9cefda
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (60/66)
....     (66/66)


/checkout/src/test/rustdoc-gui/pocket-menu.goml pocket-menu... FAILED
[ERROR] (line 72) Error: Evaluation failed: The following errors happened (for selector `#help-button .popover`): [expected `rgb(221, 221, 221)` for key `border-color`, found `rgb(224, 224, 224)`]: for command `assert-css: (
    "#help-button .popover",
    {"display": "block", "border-color": "rgb(221, 221, 221)"},


/checkout/src/test/rustdoc-gui/search-filter.goml search-filter... FAILED
[ERROR] (line 41) Error: Evaluation failed: property `value` (`all crates`) isn't equal to `All crates` for selector `#crate-search`: for command `assert-property: ("#crate-search", {"value": "All crates"})`
[ERROR] (line 51) "#search-settings" not found: for command `assert-text: ("#search-settings", "Results in all crates", STARTS_WITH)`
[ERROR] (line 70) Error: The following CSS properties still don't match: [border: (`1px solid rgb(210, 210, 210)` != `1px solid rgb(240, 240, 240)`), color: (`rgb(221, 221, 221)` != `rgb(17, 17, 17)`), background-color: (`rgb(53, 53, 53)` != `rgb(240, 240, 240)`)]: for command `wait-for-css: ("#crate-search", {
    "border": "1px solid rgb(240, 240, 240)",
    "color": "rgb(17, 17, 17)",
    "background-color": "rgb(240, 240, 240)",


/checkout/src/test/rustdoc-gui/search-result-display.goml search-result-display... FAILED
[ERROR] (line 5) Error: The following CSS selector "#search-settings" was not found: for command `wait-for: "#search-settings"`
Build completed unsuccessfully in 0:00:43
