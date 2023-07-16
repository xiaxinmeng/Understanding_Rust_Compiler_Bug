plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between e4e417953920e198f4bc1421ce9e38fd8a85fbca and 3daf8b1033df44896ca213516bf95fd679684b29
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

---- compile_test stdout ----
thread 'compile_test' panicked at '
----------------------------------------------------------------------
ERROR: Found multiple rlibs for crates: `itertools`
Try removing the stageN-tools directory or remove the following files:
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-25dc0cde4ae95530.rlib \
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-a462d7304c071790.rlib \
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-079b879cf9789047.rlib

