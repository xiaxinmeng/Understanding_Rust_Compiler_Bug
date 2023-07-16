plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 45263fc66d253f762b3880764ae48611a25bf887 and 45b1aa259abdd0d01357af38377e5386ba6ba9e8
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   Compiling cargo_metadata v0.14.0
For more information about this error, try `rustc --explain E0635`.
error: could not compile `clippy_utils` due to previous error
warning: build failed, waiting for other jobs to finish...
thread 'main' panicked at 'in-tree tool', test.rs:649:14
Build completed unsuccessfully in 0:00:11
