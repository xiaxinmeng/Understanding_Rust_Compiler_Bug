plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 92c1937a90e5b6f20fa6e87016d6869da363972e and 399fd25780a74d979796961bf2822067abacd7fe
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building rustdoc for stage2 (x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.23s
error: flag -Zrustdoc-scrape-examples does not take a value, found: `examples`
