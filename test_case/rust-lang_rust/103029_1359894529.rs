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
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
error[E0599]: no method named `inner_tokens` found for struct `P<DelimArgs>` in the current scope
   --> compiler/rustc_expand/src/expand.rs:700:68
    |
700 |                     if self.reduce_expansion_growth_limit(mac.args.inner_tokens().len()).is_err() {
    |                                                                    ^^^^^^^^^^^^ method not found in `P<DelimArgs>`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_expand` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_expand` due to previous error
