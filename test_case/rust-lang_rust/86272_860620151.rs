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
   Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
error[E0432]: unresolved import `crate::header::should_ignore_revision`
 --> src/tools/compiletest/src/header/tests.rs:4:49
  |
4 | use crate::header::{parse_normalization_string, should_ignore_revision, EarlyProps};
  |                                                 ^^^^^^^^^^^^^^^^^^^^^^ no `should_ignore_revision` in `header`

error[E0609]: no field `should_fail` on type `EarlyProps`
  --> src/tools/compiletest/src/header/tests.rs:82:36
   |
82 |     assert!(!parse_rs(&config, "").should_fail);
   |
   |
   = note: available fields are: `aux`, `aux_crate`, `revisions`

error[E0609]: no field `should_fail` on type `EarlyProps`
  --> src/tools/compiletest/src/header/tests.rs:83:49
   |
83 |     assert!(parse_rs(&config, "// should-fail").should_fail);
   |
   |
   = note: available fields are: `aux`, `aux_crate`, `revisions`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0432, E0609.
For more information about an error, try `rustc --explain E0432`.
