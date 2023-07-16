plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 01198792a608b05e624b0127e76dd0753057016c and fa5c581aeb0261592ac96ffb7acaa567c3497e55
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
 Documenting test_docs v0.1.0 (/checkout/src/test/rustdoc-gui/src/test_docs)
error[E0428]: the name `long_code_block` is defined multiple times
   --> lib.rs:129:1
    |
126 | pub mod long_code_block {}
    | ----------------------- previous definition of the module `long_code_block` here
...
129 | pub mod long_code_block {}
    | ^^^^^^^^^^^^^^^^^^^^^^^ `long_code_block` redefined here
    |
    = note: `long_code_block` must be defined only once in the type namespace of this module
error: Compilation failed, aborting rustdoc

For more information about this error, try `rustc --explain E0428`.
error: could not document `test_docs`
error: could not document `test_docs`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc --edition=2018 --crate-type lib --crate-name test_docs lib.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/debug/deps --crate-version 0.1.0` (exit status: 1)

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui"
expected success, got: exit status: 101

