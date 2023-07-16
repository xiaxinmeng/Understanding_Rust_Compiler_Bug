plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 50872bdb99c96716ee50a3b9613c395302b99767 and bae5abd8145a93b41884685fd6a431617a3e081b
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-04-05/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
Building rustbuild
    Updating crates.io index
    Updating git repository `https://github.com/tikv/jemallocator`
    Updating git submodule `https://github.com/jemalloc/jemalloc`
---
.......... (60/64)
...       (64/64)


/checkout/src/test/rustdoc-gui/search-tab-change-title-fn-sig.goml search-tab-change-title-fn-sig... FAILED
[ERROR] (line 6) Error: The following CSS selector "#titles" was not found: for command `wait-for: "#titles"`
Build completed unsuccessfully in 0:00:46
