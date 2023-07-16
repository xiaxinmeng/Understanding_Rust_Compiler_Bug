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
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error[E0433]: failed to resolve: use of undeclared type `Autoderef`
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1109:54
     |
1109 |         let Some((def_id_or_name, output, inputs)) = Autoderef::new(
     |                                                      ^^^^^^^^^ use of undeclared type `Autoderef`
For more information about this error, try `rustc --explain E0433`.
error: could not compile `rustc_trait_selection` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_trait_selection` due to previous error
