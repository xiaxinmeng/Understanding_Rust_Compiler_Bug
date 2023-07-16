plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0433]: failed to resolve: could not find `unix` in `os`
  --> compiler/rustc_codegen_ssa/src/back/link.rs:45:14
45 | use std::os::unix::prelude::OsStringExt;
45 | use std::os::unix::prelude::OsStringExt;
   |              ^^^^ could not find `unix` in `os`

error[E0599]: no function or associated item named `from_vec` found for struct `OsString` in the current scope
    --> compiler/rustc_codegen_ssa/src/back/link.rs:2454:54
     |
2454 | ...                   let name = OsString::from_vec(entry.name().to_vec());
     |                                            ^^^^^^^^ function or associated item not found in `OsString`
Some errors have detailed explanations: E0433, E0599.
For more information about an error, try `rustc --explain E0433`.
error: could not compile `rustc_codegen_ssa` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
