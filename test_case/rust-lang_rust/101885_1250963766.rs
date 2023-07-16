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
    Checking thiserror v1.0.33
error[E0609]: no field `kind` on type `&rustc_ast::Lit`
   --> src/tools/rustfmt/src/modules/visitor.rs:104:15
    |
104 |     match lit.kind {
    |
    = note: available fields are: `token_lit`, `span`

For more information about this error, try `rustc --explain E0609`.
