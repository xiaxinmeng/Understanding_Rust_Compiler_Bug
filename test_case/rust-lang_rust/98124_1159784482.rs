plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 67404f7200c13deec255ffe1146e1d2c9d0d3028 and 7f573b64297dc68dd06de94d75520880ba4d07fb
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (60/64)
...       (64/64)


/checkout/src/test/rustdoc-gui/headings.goml headings... FAILED
[ERROR] (line 109) Error: Evaluation failed: "Variants" !== "Modules": for command `assert-text: (".sidebar h3", "Modules")`

/checkout/src/test/rustdoc-gui/duplicate-macro-reexport.goml duplicate-macro-reexport... FAILED
[ERROR] (line 4) Error: The following CSS selector ".sidebar-elems .others .macro" was not found: for command `wait-for: ".sidebar-elems .others .macro"`
Build completed unsuccessfully in 0:00:43
