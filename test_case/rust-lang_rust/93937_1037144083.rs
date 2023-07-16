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
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
error[E0433]: failed to resolve: use of undeclared crate or module `io`
   --> compiler/rustc_interface/src/util.rs:349:60
    |
349 |                 info!("GetModuleHandleExW failed: {}", io::Error::last_os_error());
    |                                                            ^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use core::fmt::Error;
    |
---

error[E0433]: failed to resolve: use of undeclared crate or module `io`
   --> compiler/rustc_interface/src/util.rs:355:60
    |
355 |                 info!("GetModuleFileNameW failed: {}", io::Error::last_os_error());
    |                                                            ^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use core::fmt::Error;
    |
---

error[E0433]: failed to resolve: use of undeclared crate or module `io`
   --> compiler/rustc_interface/src/util.rs:360:59
    |
360 |                 info!("our buffer was too small? {}", io::Error::last_os_error());
    |                                                           ^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use core::fmt::Error;
    |
