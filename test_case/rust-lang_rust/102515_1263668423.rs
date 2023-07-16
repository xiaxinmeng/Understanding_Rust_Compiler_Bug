plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between f914b82a754c6d85c0a909ab152f5b611defef73 and aa7b86f9ae360519c1035f2ece35711caf624a24
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
  |
2 | #![feature(map_first_last)]
  |            ^^^^^^^^^^^^^^
  |
  = note: `-D stable-features` implied by `-D warnings`
error: could not compile `miri` due to previous error
error: could not compile `miri` due to previous error
thread 'main' panicked at 'in-tree tool', test.rs:489:14
Build completed unsuccessfully in 0:00:15
