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
    Checking rustc_span v0.0.0 (/checkout/compiler/rustc_span)
    Checking rustc_abi v0.0.0 (/checkout/compiler/rustc_abi)
    Checking rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
    Checking rustc_parse_format v0.0.0 (/checkout/compiler/rustc_parse_format)
error[E0599]: no variant or associated item named `Replace` found for enum `DesugaringKind` in the current scope
     |
1137 | pub enum DesugaringKind {
1137 | pub enum DesugaringKind {
     | ----------------------- variant or associated item `Replace` not found for this enum
...
1170 |             DesugaringKind::Replace => "drop and replace",
     |                             ^^^^^^^ variant or associated item not found in `DesugaringKind`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_span` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_span` due to previous error
