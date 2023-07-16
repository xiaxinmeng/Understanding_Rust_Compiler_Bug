plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 1cd72b734318720adb99dc72147bb8169ef76ffe and 472e726ac7b633b827a96f5760c5d3a97abb5b69
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
  |
5 | pub mod another_folder;
  | ^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: to create the module `another_folder`, create file "another_folder.rs" or "another_folder/mod.rs"

error[E0583]: file not found for module `another_mod`
  |
  |
6 | pub mod another_mod;
  |
  |
  = help: to create the module `another_mod`, create file "another_mod.rs" or "another_mod/mod.rs"
error: Compilation failed, aborting rustdoc

For more information about this error, try `rustc --explain E0583`.
error: could not document `lib2`
error: could not document `lib2`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc --edition=2018 --crate-type lib --crate-name lib2 lib.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=a429674ab6343938 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/debug/deps --extern implementors=/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/debug/deps/libimplementors-0dbc5ea28065df84.rmeta --crate-version 0.1.0` (exit status: 1)
