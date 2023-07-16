plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between ebed06fcba3b58913a5087039a81478d43b47b2f and 678c61013d73f67d4f53049bcc032e67c1dec37a
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
info: removing rustup home
info: removing cargo home
info: removing rustup binaries
info: rustup is uninstalled
find: ‘/home/runner/work/rust/rust/src/ci/docker/../src/tests/rustdoc-gui/browser-ui-test.version’: No such file or directory
##[error]Process completed with exit code 1.
