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
    Checking addr2line v0.16.0
error[E0412]: cannot find type `ControlFlow` in this scope
   --> library/std/src/sys/windows/process.rs:620:10
    |
620 |     ) -> ControlFlow<Result<Infallible, crate::process::ExitStatusError>, ()> {
    |
help: consider importing one of these items
    |
6   | use core::ops::ControlFlow;
6   | use core::ops::ControlFlow;
    |
6   | use crate::ops::ControlFlow;
    |

error[E0412]: cannot find type `Infallible` in this scope
   --> library/std/src/sys/windows/process.rs:620:29
    |
620 |     ) -> ControlFlow<Result<Infallible, crate::process::ExitStatusError>, ()> {
    |
help: consider importing one of these items
    |
6   | use core::convert::Infallible;
6   | use core::convert::Infallible;
    |
6   | use crate::convert::Infallible;
    |

error[E0433]: failed to resolve: use of undeclared type `ControlFlow`
   --> library/std/src/sys/windows/process.rs:622:28
    |
622 |             Ok(failure) => ControlFlow::Break(Err(crate::process::ExitStatusError::new(
    |
help: consider importing one of these items
    |
6   | use core::ops::ControlFlow;
6   | use core::ops::ControlFlow;
    |
6   | use crate::ops::ControlFlow;
    |

error[E0433]: failed to resolve: use of undeclared type `ControlFlow`
   --> library/std/src/sys/windows/process.rs:625:23
    |
625 |             Err(_) => ControlFlow::Continue(()),
    |
help: consider importing one of these items
    |
6   | use core::ops::ControlFlow;
