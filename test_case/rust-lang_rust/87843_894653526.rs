plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
  |
7 | use std::assert_matches::assert_matches;
  |     ^^^ use of undeclared crate or module `std`

error: cannot find macro `assert_matches` in this scope
   --> library/std/src/collections/hash/map/tests.rs:831:5
831 |     assert_matches!(
    |     ^^^^^^^^^^^^^^
    |
    = note: consider importing one of these items:
    = note: consider importing one of these items:
            core::assert_matches::assert_matches
            crate::assert_matches::assert_matches

error: cannot find macro `assert_matches` in this scope
   --> library/std/src/collections/hash/map/tests.rs:825:5
825 |     assert_matches!(
    |     ^^^^^^^^^^^^^^
    |
    = note: consider importing one of these items:
    = note: consider importing one of these items:
            core::assert_matches::assert_matches
            crate::assert_matches::assert_matches

error: unused import: `realstd::collections::TryReserveErrorKind::*`
 --> library/std/src/collections/hash/map/tests.rs:6:5
  |
6 | use realstd::collections::TryReserveErrorKind::*;
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
For more information about this error, try `rustc --explain E0658`.
error: could not compile `alloc` due to 55 previous errors
warning: build failed, waiting for other jobs to finish...
For more information about this error, try `rustc --explain E0433`.
