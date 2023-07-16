plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between fa51fc01ca3d654d08d627b1d1482d1b77e5ed8b and 676efd17014c8166d6c6dcfa9c9b22f0fb204fdf
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
.......... (90/97)
.......    (97/97)


/checkout/src/test/rustdoc-gui/rust-logo.goml rust-logo... FAILED
[ERROR] line 35: expected `,` or `}` after `ayu`, found `"`
