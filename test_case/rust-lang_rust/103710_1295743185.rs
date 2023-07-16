plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 7174231ae66aa3e938cbe0b84e23e79d867fec20 and 8e7a6be0a4641d2453f6283874079056ca8df7d8
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (80/81)
.          (81/81)


/checkout/src/test/rustdoc-gui/search-result-color.goml search-result-color... FAILED
[ERROR] (line 1 from 124) ".result-structfield .structfield" not found: for command `assert-css: (".result-" + |result_kind| + " ." + |result_kind|, {"color": |color|}, ALL)`
[ERROR] (line 2 from 124) ".result-structfield" not found: for command `assert-css: (
                ".result-" + |result_kind|,
                {"color": |entry_color|, "background-color": |background_color|},
            )`
[ERROR] (line 6 from 124) Error: No element found for selector: .result-structfield: for command `move-cursor-to: ".result-" + |result_kind|`
Build completed unsuccessfully in 0:01:52
