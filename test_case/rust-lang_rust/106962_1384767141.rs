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
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error[E0433]: failed to resolve: use of undeclared type `DiagnosticsMode`
     |
     |
2437 |                 let additional_newline = if let FoundUse::Yes = found_use && let DiagnosticsMode::Normal = mode { "" } else { "\n" };
     |                                                                                  ^^^^^^^^^^^^^^^ use of undeclared type `DiagnosticsMode`
For more information about this error, try `rustc --explain E0433`.
error: could not compile `rustc_resolve` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_resolve` due to previous error
