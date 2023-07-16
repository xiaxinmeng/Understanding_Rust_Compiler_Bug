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
    Checking rand v0.7.3
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking core v0.0.0 (/checkout/library/core)
    Checking std v0.0.0 (/checkout/library/std)
error[E0433]: failed to resolve: could not find `unix` in `os`
 --> library/std/src/process/tests.rs:5:16
5 | use crate::os::unix::process::CommandExt;
5 | use crate::os::unix::process::CommandExt;
  |                ^^^^ could not find `unix` in `os`

error[E0599]: no method named `arg0` found for struct `process::Command` in the current scope
   --> library/std/src/process/tests.rs:445:13
    |
445 |     command.arg0("exciting-name");
    |             ^^^^ help: there is an associated function with a similar name: `arg`
   ::: library/std/src/process.rs:513:1
    |
513 | pub struct Command {
513 | pub struct Command {
    | ------------------ method `arg0` not found for this
Some errors have detailed explanations: E0433, E0599.
For more information about an error, try `rustc --explain E0433`.
error: could not compile `std` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
