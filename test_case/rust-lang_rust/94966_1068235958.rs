plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between be52b4af5ec7e49572cb16519b7e144d6bcb051d and 67d451a00586653be47bd535799cdceef225e869
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (60/62)
.         (62/62)


/checkout/src/test/rustdoc-gui/escape-key.goml escape-key... FAILED
[ERROR] (line 6) TimeoutError: waiting for selector "#search h1" failed: timeout 30000ms exceeded: for command `wait-for: "#search h1" // The search element is empty before the first search `
[ERROR] (line 7) Error: Evaluation failed: expected `content` for attribute `class` for selector `#search`, found `content hidden`: for command `assert-attribute: ("#search", {"class": "content"})`
[ERROR] (line 8) Error: Evaluation failed: expected `content hidden` for attribute `class` for selector `#main-content`, found `content`: for command `assert-attribute: ("#main-content", {"class": "content hidden"})`
Build completed unsuccessfully in 0:00:42
