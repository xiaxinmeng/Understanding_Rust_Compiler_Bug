plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
   Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
error[E0432]: unresolved import `crate::read2::EXCLUDED_PLACEHOLDER_LEN`
 --> src/tools/compiletest/src/read2/tests.rs:1:32
  |
1 | use crate::read2::{ProcOutput, EXCLUDED_PLACEHOLDER_LEN, HEAD_LEN, TAIL_LEN};
  |                                ^^^^^^^^^^^^^^^^^^^^^^^^ no `EXCLUDED_PLACEHOLDER_LEN` in `read2`

error[E0026]: variant `ProcOutput::Full` does not have a field named `excluded_len`
   |
   |
73 |         ProcOutput::Full { excluded_len, .. } => assert_eq!(
   |                            ^^^^^^^^^^^^ variant `ProcOutput::Full` does not have this field
Some errors have detailed explanations: E0026, E0432.
For more information about an error, try `rustc --explain E0026`.
error: could not compile `compiletest` due to 2 previous errors
Build completed unsuccessfully in 0:01:24
